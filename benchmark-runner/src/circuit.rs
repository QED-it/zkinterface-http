use failure::Error;
use std::io;
use std::mem::size_of;
use zkinterface::{
    examples::serialize_small,
    flatbuffers::{emplace_scalar, EndianScalar, FlatBufferBuilder},
    writing::{CircuitOwned, VariablesOwned},
    zkinterface_generated::zkinterface::{
        BilinearConstraint,
        BilinearConstraintArgs,
        Message,
        R1CSConstraints,
        R1CSConstraintsArgs,
        Root,
        RootArgs,
        Variables,
        VariablesArgs,
        Witness,
        WitnessArgs,
    },
};

const VARS_PER_CONSTRAINT: u64 = 4;

pub fn make_benchmark_circuit(num_inputs: u64, num_constraints: u64) -> Result<Vec<u8>, Error> {
    let circuit = CircuitOwned {
        connections: VariablesOwned {
            variable_ids: vec![0],
            values: Some(serialize_small(&[1 as u64])),
        },
        free_variable_id: 1 + VARS_PER_CONSTRAINT * num_constraints,
        r1cs_generation: true,
        field_maximum: None,
    };

    let mut buf = Vec::<u8>::new();

    circuit.write(&mut buf)?;
    write_constraints(&mut buf, num_constraints)?;
    write_witness(&mut buf, num_constraints)?;

    Ok(buf)
}

pub fn write_constraints<W: io::Write>(mut writer: W, num_constraints: u64) -> Result<(), Error> {
    let mut builder = &mut FlatBufferBuilder::new();
    let mut constraints_built = vec![];

    // Create constraints of this form:
    //     (var_1 + var_2) * var_3 = var_4
    for i in 0..num_constraints {
        let id_offset = i * VARS_PER_CONSTRAINT;
        let lca = VariablesOwned {
            variable_ids: vec![id_offset + 1, id_offset + 2],
            values: Some(vec![1]),
        }.build(builder);
        let lcb = VariablesOwned {
            variable_ids: vec![id_offset + 3],
            values: Some(vec![1]),
        }.build(builder);
        let lcc = VariablesOwned {
            variable_ids: vec![id_offset + 4],
            values: Some(vec![1]),
        }.build(builder);

        constraints_built.push(BilinearConstraint::create(builder, &BilinearConstraintArgs {
            linear_combination_a: Some(lca),
            linear_combination_b: Some(lcb),
            linear_combination_c: Some(lcc),
        }));
    }

    let constraints_built = builder.create_vector(&constraints_built);
    let r1cs = R1CSConstraints::create(&mut builder, &R1CSConstraintsArgs {
        constraints: Some(constraints_built),
        info: None,
    });

    let message = Root::create(&mut builder, &RootArgs {
        message_type: Message::R1CSConstraints,
        message: Some(r1cs.as_union_value()),
    });
    builder.finish_size_prefixed(message, None);

    writer.write_all(builder.finished_data())?;

    Ok(())
}

pub fn write_witness<W: io::Write>(mut writer: W, num_constraints: u64) -> Result<(), Error> {
    let mut ids = Vec::<u64>::new();
    let mut values = Vec::<u64>::new();

    // Create triplets of values of this form:
    //     (var_1 + var_2) * var_3 = var_4
    //     (  2   +   3  ) *   4   =   20
    for i in 0..num_constraints {
        let id = i * VARS_PER_CONSTRAINT;
        ids.push(id + 1);
        values.push(2);
        ids.push(id + 2);
        values.push(3);
        ids.push(id + 3);
        values.push(4);
        ids.push(id + 4);
        values.push(20);
    }

    let mut builder = &mut FlatBufferBuilder::new();
    let ids = builder.create_vector(&ids);
    let values = serialize_small(&values);
    let values = builder.create_vector(&values);

    let values = Variables::create(&mut builder, &VariablesArgs {
        variable_ids: Some(ids),
        values: Some(values),
        info: None,
    });
    let assign = Witness::create(&mut builder, &WitnessArgs {
        assigned_variables: Some(values),
    });
    let message = Root::create(&mut builder, &RootArgs {
        message_type: Message::Witness,
        message: Some(assign.as_union_value()),
    });
    builder.finish_size_prefixed(message, None);

    writer.write_all(builder.finished_data())?;

    Ok(())
}
