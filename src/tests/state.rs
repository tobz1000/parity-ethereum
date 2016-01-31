use super::test_common::*;
use pod_state::*;
use state_diff::*;
use ethereum;
use tests::helpers::*;

fn do_json_test(json_data: &[u8]) -> Vec<String> {
	let json = Json::from_str(::std::str::from_utf8(json_data).unwrap()).expect("Json is invalid");
	let mut failed = Vec::new();

	let engine = ethereum::new_frontier_like_test().to_engine().unwrap();
	flush(format!("\n"));

	for (name, test) in json.as_object().unwrap() {
		let mut fail = false;
		{
			let mut fail_unless = |cond: bool| if !cond && !fail {
				failed.push(name.clone());
				flush(format!("FAIL\n"));
				fail = true;
				true
			} else {false};

			flush(format!("   - {}...", name));

			let t = Transaction::from_json(&test["transaction"]);
			let env = EnvInfo::from_json(&test["env"]);
			let _out = Bytes::from_json(&test["out"]);
			let post_state_root = xjson!(&test["postStateRoot"]);
			let pre = PodState::from_json(&test["pre"]);
			let post = PodState::from_json(&test["post"]);
			let logs: Vec<_> = test["logs"].as_array().unwrap().iter().map(&LogEntry::from_json).collect();

			//println!("Transaction: {:?}", t);
			//println!("Env: {:?}", env);
			let calc_post = sec_trie_root(post.get().iter().map(|(k, v)| (k.to_vec(), v.rlp())).collect());

			if fail_unless(post_state_root == calc_post) {
				println!("!!! {}: Trie root mismatch (got: {}, expect: {}):", name, calc_post, post_state_root);
				println!("!!! Post:\n{}", post);
			} else {
				let mut state_result = get_temp_state();
				let mut state = state_result.reference_mut();
				state.populate_from(pre);
				state.commit();
				let res = state.apply(&env, engine.deref(), &t);

				if fail_unless(state.root() == &post_state_root) {
					println!("!!! {}: State mismatch (got: {}, expect: {}):", name, state.root(), post_state_root);
					let our_post = state.to_pod();
					println!("Got:\n{}", our_post);
					println!("Expect:\n{}", post);
					println!("Diff ---expect -> +++got:\n{}", StateDiff::diff_pod(&post, &our_post));
				}

				if let Ok(r) = res {
					if fail_unless(logs == r.logs) {
						println!("!!! {}: Logs mismatch:", name);
						println!("Got:\n{:?}", r.logs);
						println!("Expect:\n{:?}", logs);
					}
				}
			}
		}
		if !fail {
			flush(format!("ok\n"));
		}
		// TODO: Add extra APIs for output
		//if fail_unless(out == r.)
	}
	println!("!!! {:?} tests from failed.", failed.len());
	failed
}

declare_test!{StateTests_stBlockHashTest, "StateTests/stBlockHashTest"}
declare_test!{StateTests_stCallCodes, "StateTests/stCallCodes"}
declare_test!{StateTests_stCallCreateCallCodeTest, "StateTests/stCallCreateCallCodeTest"}
declare_test!{StateTests_stDelegatecallTest, "StateTests/stDelegatecallTest"}
declare_test!{StateTests_stExample, "StateTests/stExample"}
declare_test!{StateTests_stInitCodeTest, "StateTests/stInitCodeTest"}
declare_test!{StateTests_stLogTests, "StateTests/stLogTests"}
declare_test!{heavy => StateTests_stMemoryStressTest, "StateTests/stMemoryStressTest"}
declare_test!{heavy => StateTests_stMemoryTest, "StateTests/stMemoryTest"}
declare_test!{StateTests_stPreCompiledContracts, "StateTests/stPreCompiledContracts"}
declare_test!{heavy => StateTests_stQuadraticComplexityTest, "StateTests/stQuadraticComplexityTest"}
declare_test!{StateTests_stRecursiveCreate, "StateTests/stRecursiveCreate"}
declare_test!{StateTests_stRefundTest, "StateTests/stRefundTest"}
declare_test!{StateTests_stSolidityTest, "StateTests/stSolidityTest"}
declare_test!{StateTests_stSpecialTest, "StateTests/stSpecialTest"}
declare_test!{StateTests_stSystemOperationsTest, "StateTests/stSystemOperationsTest"}
declare_test!{StateTests_stTransactionTest, "StateTests/stTransactionTest"}
declare_test!{StateTests_stTransitionTest, "StateTests/stTransitionTest"}
declare_test!{StateTests_stWalletTest, "StateTests/stWalletTest"}
