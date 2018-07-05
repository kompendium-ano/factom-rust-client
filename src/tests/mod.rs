use super::*; 

fn factomd()-> Factomd{
    let mut factomd = Factomd::new();
    factomd.port(443);
    factomd.https();
    factomd
}

fn get_result<F, R, E>(fut: F)-> Result<R, E>
    where
        F: Send + 'static + Future<Item = R, Error = E>,
        R: Send + 'static,
        E: Send + 'static,
    {
        let mut runtime = tokio::runtime::Runtime::new().expect("Unable to create a tokio runtime");
        runtime.block_on(fut)
    }

#[test]
fn ablock_by_height() {
    let query = factomd().ablock_by_height(14460)
                            .map(|result| {
                                result.result["ablock"]["backreferencehash"].clone()
                            })
                            .map_err(|err| err);
    let expected = "0a9aa1efbe7d0e8d9c1d460d1c78e3e7b50f984e65a3f3ee7b73100a94189dbf";
    let result = get_result(query);
    assert_eq!(result.unwrap(), expected);
}