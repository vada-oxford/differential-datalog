fn main() {

    //
    // p(X,Z) :- q(Y,X), r(Y,Z).
    //
    timely::execute_from_args(std::env::args(), |worker| {

        let (mut r_edb_input, mut q_edb_input) =
        worker.dataflow::<usize,_,_>(|scope| {
            let (r_input, r) = scope.new_collection();
            let (q_input, q) = scope.new_collection();

            let p_idb_tuples =
            q
                .join(&r)
                .map(|(_, pair_x_z)| pair_x_z)
                .inspect(|x| println!("q: {:?}", x))
                ;

            (r_input, q_input)
        });

        let mut r_tuple_vec = Vec::<(String, String)>::new();
        let mut q_tuple_vec = Vec::<(String, String)>::new();

        r_tuple_vec.push((String::from("1"), String::from("Lukas")));
        r_tuple_vec.push((String::from("2"), String::from("Gerald")));
        r_tuple_vec.push((String::from("3"), String::from("Evgeny")));

        q_tuple_vec.push((String::from("1"), String::from("Simone")));
        q_tuple_vec.push((String::from("2"), String::from("Sabine")));
        q_tuple_vec.push((String::from("3"), String::from("Alina")));


        for r_tuple in r_tuple_vec {
            r_edb_input.insert(r_tuple);
        }

        for q_tuple in q_tuple_vec {
            q_edb_input.insert(q_tuple);
        }

        loop {
            let line: String = read!("{}\n");

            if line.starts_with('q') {

            } else if line.starts_with('r') {

            }
        }
    }).unwrap();