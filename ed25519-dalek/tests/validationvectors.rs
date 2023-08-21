pub const VALIDATIONVECTORS: &str = r#"[
	{
		"_comment": "This test vector comes from https://github.com/C2SP/CCTV/blob/5ea85644bd035c555900a2f707f7e4c31ea65ced/ed25519vectors/ed25519vectors.json",
		"number": 0,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 1,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 11",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 2,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 3,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 4,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "00000000000000000000000000000000000000000000000000000000000000005e176f12cfb0d4e6eb6929b19ae4c998ef05c1c2cf628a9b1fa1c21312627108",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 5,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "00000000000000000000000000000000000000000000000000000000000000009472a69cd9a701a50d130ed52189e2455b23767db52cacb8716fb896ffeeac09",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 6,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56bc02e2b9e63e385c058bf62b14b3a2b29ccefe8e38ddb536bc3f9865320a3d801",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 7,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56bbbfd00bd9c259d8d222d15e67a3d8228585050dbb9b9585be20d8fadc721da03",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 8,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 9,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_A"
		]
	},
	{
		"number": 10,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 11,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_A"
		]
	},
	{
		"number": 12,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 13,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 14,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 15,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 16,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f1cb421dfbd92aa6c30d550bff53c81cf650ace6deb96a8ec22d2fef84dbbe20b",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 17,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7fad7a355469b5c87e550469f6b2de409ee723acd584bf35f86b80c384e8ceb702",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 18,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f5e176f12cfb0d4e6eb6929b19ae4c998ef05c1c2cf628a9b1fa1c21312627108",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 19,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f9472a69cd9a701a50d130ed52189e2455b23767db52cacb8716fb896ffeeac09",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 20,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 21,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 15",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 22,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "00000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 23,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "00000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 24,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 25,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 26,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "000000000000000000000000000000000000000000000000000000000000008075b3d7e9547febbdbf3fde21df901c7ca1fc59e8b689a4ae283919e78cf62b03",
		"msg": "ed25519vectors 11",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 27,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "0000000000000000000000000000000000000000000000000000000000000080050abffcd4d8ccbb4b8d6bf6649f5aa99e8de5cc182a7409633856ff53f49e0c",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 28,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a94639734cf989e314e9e049fc01a3864d191fed8f231b12fee6fa50aaadba44b0e",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 29,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a94732d9e0279fe001d90327efa319816e3e4506b78432b8b4e1f2fdc4b960d700f",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 30,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "00000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 31,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "00000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_A"
		]
	},
	{
		"number": 32,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 33,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_A"
		]
	},
	{
		"number": 34,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 35,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 36,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 37,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 38,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7117c2ec783065c50f2c930cfc9d318c9737991acb260b49e2ca815474532709",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 39,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff1f5fae114eb0c534b5aed0a892d0a0e1d429df6a025c5ae08012e0ffce78310c",
		"msg": "ed25519vectors 27",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 40,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff75b3d7e9547febbdbf3fde21df901c7ca1fc59e8b689a4ae283919e78cf62b03",
		"msg": "ed25519vectors 11",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 41,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff050abffcd4d8ccbb4b8d6bf6649f5aa99e8de5cc182a7409633856ff53f49e0c",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 42,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 39",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 43,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 44,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 11",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A"
		]
	},
	{
		"number": 45,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 46,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_A",
			"low_order_component_A"
		]
	},
	{
		"number": 47,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_A",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 48,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "0100000000000000000000000000000000000000000000000000000000000000edfd6f478b59eab2545cf8f3d48c6574d2b4e8abd948148da5c62479113b8d0f",
		"msg": "ed25519vectors 13",
		"flags": [
			"low_order_R",
			"low_order_component_A"
		]
	},
	{
		"number": 49,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "0100000000000000000000000000000000000000000000000000000000000000879e8751046a31d163df68051bbd909e6d26c8414883ba3475bf00e17c1e7b04",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 50,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350608f32d206a7c0b7efa9a59e66546e8f1f599ef843fb502c9cc3c4ae8b7c11e05",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_component_A"
		]
	},
	{
		"number": 51,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a3506030b03796b78f7afeadfccaedc9d09ce6d487d1ece1f16b1ae2b59b7e5c40603",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 52,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_A"
		]
	},
	{
		"number": 53,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 54,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_A",
			"low_order_component_A",
			"non_canonical_A"
		]
	},
	{
		"number": 55,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 56,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 57,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 58,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "01000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 59,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "01000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 60,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "0100000000000000000000000000000000000000000000000000000000000080c85efbb96e35e1b671722d5d8de687d4e148ea15ec566be6a1f3cfb8a10a9d06",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 61,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "01000000000000000000000000000000000000000000000000000000000000804b18627cef9707137b02358c8b73769381269bf7cac9c473483c83c46a615d09",
		"msg": "ed25519vectors 29",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 62,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "0100000000000000000000000000000000000000000000000000000000000080edfd6f478b59eab2545cf8f3d48c6574d2b4e8abd948148da5c62479113b8d0f",
		"msg": "ed25519vectors 13",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 63,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "0100000000000000000000000000000000000000000000000000000000000080879e8751046a31d163df68051bbd909e6d26c8414883ba3475bf00e17c1e7b04",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 64,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 65,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 66,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f97f5fc03f658b5b733cf20c4ea5577e8e5988ee90cb2a3c919c2a05dd2fcde04",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 67,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f2786a72a405895f7b3b4752eac49c8973270173a495ec475b34933dc05d7e904",
		"msg": "ed25519vectors 16",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 68,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7fedfd6f478b59eab2545cf8f3d48c6574d2b4e8abd948148da5c62479113b8d0f",
		"msg": "ed25519vectors 13",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 69,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f879e8751046a31d163df68051bbd909e6d26c8414883ba3475bf00e17c1e7b04",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 70,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 13",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 71,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 72,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff87a0755495e4b763c07eeebdd510eb7d7b167bf13bf1489785fa2be61543890e",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 73,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffafdefad9d081387fa502d650442ed15619fc936d41444fd5c4292691a16f1d06",
		"msg": "ed25519vectors 25",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 74,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffedfd6f478b59eab2545cf8f3d48c6574d2b4e8abd948148da5c62479113b8d0f",
		"msg": "ed25519vectors 13",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 75,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff879e8751046a31d163df68051bbd909e6d26c8414883ba3475bf00e17c1e7b04",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 76,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "01000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 11",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 77,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "01000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 78,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 15",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 79,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 22",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 80,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 81,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 82,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc050000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 83,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d2105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 84,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05aa80ce94a6f94b9e01abfc089182b3d85548437339d7ad2e3d804f60a87a8605",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 85,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d21bba68b891fdce7ad8fa923bc884aa33eeea4f341ae5ee527cb7d99a23040ab0e",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 86,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc050000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 21",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 87,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d2105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 88,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d2105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 89,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc850000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 90,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f1105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 91,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc852cd7e84d7f4609fc08f069561f201161369e38e508562ea21f0c6f582873f500",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 92,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f11efaa29ea31bbaf7b896625bd0fbe78f6bb7f3cf093407794dd2cd6096e3fd103",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 93,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc850000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 94,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f1105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 95,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f1105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 96,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 97,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 98,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a57f5931a402c5cda578690e33a33ba0458eec51036b5c01c5cd486a58c0d290f",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 99,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee8093711b51dce8d4ef35fd239caecd236d2ca58604bc779880708568423a9700",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 100,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 23",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 101,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 102,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 103,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 104,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 105,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa6913952aa42c06cd8759d2175fa60c29fba5d9767291b8cda0f58d186320d604",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 106,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de0b20981a60242434fb7351f2ebc29b90bbd45c90eae5377379d780156e297409",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 107,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 14",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 108,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 109,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 110,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 111,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 112,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 17",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 113,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 114,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f5639fc07db8ae613081841876d58857e3014e5f2efdeec154ae0318b2586f601",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 115,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f1509e3a68b74a8fd2b4e271b7c584a0e3cb857b6c3473e8fdf3436a2a2d0cc04",
		"msg": "ed25519vectors 31",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 116,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf9f09162ea121fb3e2d1270dd30ade8b5bcfdcfd1dcf624ac22cf60af9610c6c02",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 117,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf9ecf8dc78bf5c647c714a00acf11719551952b488c2c9df967c7d48b479420e0e",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 118,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 119,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_A"
		]
	},
	{
		"number": 120,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 121,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 20",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_A"
		]
	},
	{
		"number": 122,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 17",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 123,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 124,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 39",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 125,
		"key": "0000000000000000000000000000000000000000000000000000000000000000",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 126,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff24d6a9e426c0871c55d7163f0fe34e776bec47e582548d9bba074102c81fce02",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 127,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff35036672b82d501137113e048abbd4f44090e3aaa7e262f555e3e78fec78e507",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 128,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff5639fc07db8ae613081841876d58857e3014e5f2efdeec154ae0318b2586f601",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 129,
		"key": "10eb7c3acfb2bed3e0d6ab89bf5a3d6afddd1176ce4812e38d9fd485058fdb1f",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff1509e3a68b74a8fd2b4e271b7c584a0e3cb857b6c3473e8fdf3436a2a2d0cc04",
		"msg": "ed25519vectors 31",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 130,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 131,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 132,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 133,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 134,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 135,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 136,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "00000000000000000000000000000000000000000000000000000000000000005635c691e820339382bb85c151038e4b1815ceaaeb76bfa90cb7bf3382ec510b",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 137,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "0000000000000000000000000000000000000000000000000000000000000000d2abf247722e6b4213f1890beb32c730b02a929e22a1a8de5fe84fa1e8bec40b",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 138,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b2d155343b4933733be02419ac0ad255666ea0ad80d9998cc7c6086f4cf453507",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 139,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b25e180a8f97db7419c7adb79f5063f045cb18fc6af517852e3687be84bd7e803",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 140,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 141,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_A"
		]
	},
	{
		"number": 142,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 11",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 143,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 27",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_A"
		]
	},
	{
		"number": 144,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 145,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 146,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 21",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 147,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 148,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f9b8a1787d3747a0740ed283e74e2c478d4b681f0aa2676a5c694889696138205",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 149,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f938dd30260e6bce7f9422c05def45ba6732946a9f1236aff3a187902d7128808",
		"msg": "ed25519vectors 14",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 150,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f5635c691e820339382bb85c151038e4b1815ceaaeb76bfa90cb7bf3382ec510b",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 151,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7fd2abf247722e6b4213f1890beb32c730b02a929e22a1a8de5fe84fa1e8bec40b",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 152,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 153,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 14",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 154,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "00000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 11",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 155,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "00000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 156,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 157,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 158,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "0000000000000000000000000000000000000000000000000000000000000080fc5d42211dbee053d3bac3913e05f32c8026274b1926b55706c621eefaa7c805",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 159,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "00000000000000000000000000000000000000000000000000000000000000807ecde75a10d2cf5c13c49b78e4026148484c48d151616ae934cf4ec66f0e3702",
		"msg": "ed25519vectors 16",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 160,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9468966bbfc7bcce00aeebb561f0197cb0b823ec3f17056333fce73486dc3cbe06",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 161,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9426c8d0de0953df46bc0049fed1261b7c06189e84e5c14985454485ba5a4fb501",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 162,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "00000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 163,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "00000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_A"
		]
	},
	{
		"number": 164,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 165,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_A"
		]
	},
	{
		"number": 166,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 167,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 168,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 169,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 13",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 170,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff28d80ed8be5636443022fa8a24dbe8f649bea0e6edc5a6a65b29db14caa1b80e",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 171,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff3cb05c8a71d4d341e74340536fea96cd93577c8044879400da68b577ed7d7207",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 172,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "edfffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc5d42211dbee053d3bac3913e05f32c8026274b1926b55706c621eefaa7c805",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 173,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7ecde75a10d2cf5c13c49b78e4026148484c48d151616ae934cf4ec66f0e3702",
		"msg": "ed25519vectors 16",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 174,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 175,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 176,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A"
		]
	},
	{
		"number": 177,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 178,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 15",
		"flags": [
			"low_order_A",
			"low_order_component_A"
		]
	},
	{
		"number": 179,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_A",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 180,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "0100000000000000000000000000000000000000000000000000000000000000164aae8ae8d165867053dd1c3806319e064523aa053b1d7003463de8c5fcba0f",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_component_A"
		]
	},
	{
		"number": 181,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "0100000000000000000000000000000000000000000000000000000000000000f94cd85e18f5b89bfc21b4d29b43e30038f84228fe81eff5b7d735073ba29909",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 182,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a35063cb743a485cffaa684d0d66a5ae347d17a5638e90dc2be541023114a5961f805",
		"msg": "ed25519vectors 14",
		"flags": [
			"low_order_component_A"
		]
	},
	{
		"number": 183,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a35061fc409b236539503e78560d6183d748c8a6d3e635e87c9397531394f3cb3f902",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 184,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_A"
		]
	},
	{
		"number": 185,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 186,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 34",
		"flags": [
			"low_order_A",
			"low_order_component_A",
			"non_canonical_A"
		]
	},
	{
		"number": 187,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 188,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 15",
		"flags": [
			"low_order_A",
			"low_order_component_A",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 189,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 190,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "01000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 191,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "01000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 192,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "0100000000000000000000000000000000000000000000000000000000000080001bb260c36aea3c94ff5785b5a44d99a88c33e02dceb9fdfe9ae57aad604b03",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 193,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "0100000000000000000000000000000000000000000000000000000000000080b38e0b9773175cdae4741a4973fa1ac1bbcd7263c5f28a3cf2a5bedca4d31106",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 194,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "0100000000000000000000000000000000000000000000000000000000000080164aae8ae8d165867053dd1c3806319e064523aa053b1d7003463de8c5fcba0f",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 195,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "0100000000000000000000000000000000000000000000000000000000000080f94cd85e18f5b89bfc21b4d29b43e30038f84228fe81eff5b7d735073ba29909",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 196,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 14",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 197,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 198,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f522d2c5eddb07da5dc5f2207a513322ee5860d4b5f94103ca604060d3672f402",
		"msg": "ed25519vectors 24",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 199,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7ffba7ac98b395cf8ffaecd3676b362f25f404252b7cbf07837b6bf780d2897b0c",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 200,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f164aae8ae8d165867053dd1c3806319e064523aa053b1d7003463de8c5fcba0f",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 201,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7ff94cd85e18f5b89bfc21b4d29b43e30038f84228fe81eff5b7d735073ba29909",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 202,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 203,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 204,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe29cfd8591462aa4ba19c79672667e0665bfb240f29de8ab80b43497f12b340e",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 205,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffbe2295259b59b14675c8403202c2fe88c9d9eaf761f00f67db0dd24b67b09b01",
		"msg": "ed25519vectors 11",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 206,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff164aae8ae8d165867053dd1c3806319e064523aa053b1d7003463de8c5fcba0f",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 207,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "eefffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff94cd85e18f5b89bfc21b4d29b43e30038f84228fe81eff5b7d735073ba29909",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 208,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "01000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 209,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "01000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 210,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 22",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 211,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 212,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 213,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 214,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc050000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 215,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d2105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 216,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05486fb2a5d2cb002b460db9c56c36954aebaa4e6062f300beebb2749e32274106",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 217,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d21b9d3fadc566a3fb952d94b93f7add4e9e017bcf843058986b5ec9bec0638ba05",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 218,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc050000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 219,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d2105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 220,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d2105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 221,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc850000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 222,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f1105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 223,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85b4f602297e877cbbe446de71c9868e98c04a56e9bc409f60beaa400e7c633600",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 224,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f11a106ed973e5cdc868eace49e639ee52f4ca53fc715f8ec612e0917f4204f920e",
		"msg": "ed25519vectors 13",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 225,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc850000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 226,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f1105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 227,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f1105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 228,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 229,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 230,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a1873e9ebb328202c033407e8b9f67e528093ea899da61e9ede6999b358de0f08",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 231,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee1eb53c48312206ed56a5f83ef3fd7b6ca3012fd4c2fb08aceea83ad251358000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 232,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 233,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 234,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 235,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 236,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 237,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa0826b7ce39c8491fabdd6759f89e5a424c344ecb7d94636b268e61a2a90ab20a",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 238,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72deff8aa11cfa5ea34bc35398cf040ad303d96d5b5bd818963f69cdda872d536904",
		"msg": "ed25519vectors 17",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 239,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 240,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 241,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 242,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 13",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 243,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 244,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 245,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 38",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 246,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f109d35eafa88df6c5770dd5ef7023b3965891d0c4050d2e9c7906613229d6c0e",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 247,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f86e5c516a446a3c8e69728091c2be6eb479dfe18bb36efd34e08fd8acc730407",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 248,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf99f45ef3994a163d7cd58c2cead128afd1e3f3bc1a8ff98cbf1bc35ed609d420b",
		"msg": "ed25519vectors 12",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 249,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf96f525d28c69be17ff4c0922ec4de39da15d46baad32dde92e4c37201b5cbd103",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 250,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 251,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 21",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_A"
		]
	},
	{
		"number": 252,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 253,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 16",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_A"
		]
	},
	{
		"number": 254,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 255,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 38",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 256,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 257,
		"key": "0000000000000000000000000000000000000000000000000000000000000080",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 258,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffb2323a8aa3bc6edc5f2f3590ba2db757d50bf47c9f16cd1fd6a7db0f4ca24e0b",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 259,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa9189abccf6d929dc864b3eec19e26e4cc94b244565779a7f85ddb8e03c3c108",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 260,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff109d35eafa88df6c5770dd5ef7023b3965891d0c4050d2e9c7906613229d6c0e",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 261,
		"key": "dd1483c5304d412c1f29547640a5c2950222ee8931b7ed1c72602b7afa7024e0",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff86e5c516a446a3c8e69728091c2be6eb479dfe18bb36efd34e08fd8acc730407",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 262,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 263,
		"key": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 264,
		"key": "0100000000000000000000000000000000000000000000000000000000000000",
		"sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 265,
		"key": "0100000000000000000000000000000000000000000000000000000000000000",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 266,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "0000000000000000000000000000000000000000000000000000000000000000ef1195f0610612912016b3ee33054829bcea20ae4de1e9343b1c4e2e15649003",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 267,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b2d471882773a677af1e3824e757a33f8ddf7bbeaf3d28a09595eb8daa0d74a03",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 268,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 269,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 20",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 270,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 271,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 272,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 273,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 274,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 275,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 25",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 276,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 277,
		"key": "0100000000000000000000000000000000000000000000000000000000000000",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 278,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7fa83f56d4ca92da1f056c0bbaa0cc73cb350790210431ad647415c6a71242860c",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 279,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7fef1195f0610612912016b3ee33054829bcea20ae4de1e9343b1c4e2e15649003",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 280,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 281,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 17",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 282,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 283,
		"key": "0100000000000000000000000000000000000000000000000000000000000000",
		"sig": "00000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 30",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 284,
		"key": "0100000000000000000000000000000000000000000000000000000000000000",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 285,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "00000000000000000000000000000000000000000000000000000000000000800a4940ad86a3df965346037e94d0796fa785353be5f5d25b5c3c1a507c63bf0d",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 286,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9426cc293b14d304864a4cd689cbcc23cb13fcb1098f9c434cc9ca7439c9cf2505",
		"msg": "ed25519vectors 11",
		"flags": [
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 287,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "00000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 288,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 289,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 290,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "00000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 291,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 292,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 293,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "00000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 294,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 295,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 296,
		"key": "0100000000000000000000000000000000000000000000000000000000000000",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 297,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff944508fdd2e7908fed1b0bbbef3342fd911990128cbaa8714b0d28a617d4b508",
		"msg": "ed25519vectors 19",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 298,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0a4940ad86a3df965346037e94d0796fa785353be5f5d25b5c3c1a507c63bf0d",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 299,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 300,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 301,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 302,
		"key": "0100000000000000000000000000000000000000000000000000000000000000",
		"sig": "01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 23",
		"flags": [
			"low_order_R",
			"low_order_A"
		]
	},
	{
		"number": 303,
		"key": "0100000000000000000000000000000000000000000000000000000000000000",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_A"
		]
	},
	{
		"number": 304,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "0100000000000000000000000000000000000000000000000000000000000000330b30d6c1f07511e2208d233b6a34d4ed7109da2dea615bcd97daee3a842704",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R"
		]
	},
	{
		"number": 305,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350640657f1cb030f12543549bbabd848dc6403d354e7c99c4d0dcee9f63c9033b00",
		"msg": "ed25519vectors 5",
		"flags": null
	},
	{
		"number": 306,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_A",
			"non_canonical_A"
		]
	},
	{
		"number": 307,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_A",
			"non_canonical_A"
		]
	},
	{
		"number": 308,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_A",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 309,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 16",
		"flags": [
			"low_order_R",
			"low_order_A",
			"non_canonical_A"
		]
	},
	{
		"number": 310,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_A",
			"non_canonical_A"
		]
	},
	{
		"number": 311,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_A",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 312,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"non_canonical_A"
		]
	},
	{
		"number": 313,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_A",
			"non_canonical_A"
		]
	},
	{
		"number": 314,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_A",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 315,
		"key": "0100000000000000000000000000000000000000000000000000000000000000",
		"sig": "01000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"non_canonical_R"
		]
	},
	{
		"number": 316,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "01000000000000000000000000000000000000000000000000000000000000802d32f9eeccaa31dba2ff4ab4e196b61e2ee1648073bbb5f599dcff4f7ea02900",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"non_canonical_R"
		]
	},
	{
		"number": 317,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "0100000000000000000000000000000000000000000000000000000000000080330b30d6c1f07511e2208d233b6a34d4ed7109da2dea615bcd97daee3a842704",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 318,
		"key": "0100000000000000000000000000000000000000000000000000000000000000",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_A",
			"non_canonical_R"
		]
	},
	{
		"number": 319,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f367b7a92fea8539e8793cf812e799e8ed342c635592e74323a8dfd38a06aea0b",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"non_canonical_R"
		]
	},
	{
		"number": 320,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f330b30d6c1f07511e2208d233b6a34d4ed7109da2dea615bcd97daee3a842704",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 321,
		"key": "0100000000000000000000000000000000000000000000000000000000000000",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 18",
		"flags": [
			"low_order_R",
			"low_order_A",
			"non_canonical_R"
		]
	},
	{
		"number": 322,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "eefffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8ee37866245b5ef5bb4863572672809b98d93997190c514ccff477ebcf82f0d",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"non_canonical_R"
		]
	},
	{
		"number": 323,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff330b30d6c1f07511e2208d233b6a34d4ed7109da2dea615bcd97daee3a842704",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 324,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "01000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 325,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 326,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 11",
		"flags": [
			"low_order_R",
			"low_order_A",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 327,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "01000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_A",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 328,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 329,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 17",
		"flags": [
			"low_order_R",
			"low_order_A",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 330,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "01000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 331,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 332,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 19",
		"flags": [
			"low_order_R",
			"low_order_A",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 333,
		"key": "0100000000000000000000000000000000000000000000000000000000000000",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc050000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 334,
		"key": "0100000000000000000000000000000000000000000000000000000000000000",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d2105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 335,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05aac16c21847c8f77ede0e03571488e65c01bc418470acebc90a5f65a69c3c60f",
		"msg": "ed25519vectors 17",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 336,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d21fd3eacd8d7a35408e669bbf20fa791fc50be2a753015c51a7ff9554c4c540902",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 337,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc050000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 338,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d2105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 339,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d2105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 340,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc050000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 341,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d2105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 342,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d2105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 343,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc050000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 344,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d2105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 345,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d2105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 346,
		"key": "0100000000000000000000000000000000000000000000000000000000000000",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc850000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 347,
		"key": "0100000000000000000000000000000000000000000000000000000000000000",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f1105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 17",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 348,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc8578b7a7a27ec214b774ab6434c7afe22bc3e6fa5f12d24e81dfbd99085789170b",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 349,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f114aa4b690ec5d1502a0c77d89ede55349d93b2f4df581bc979f8f0adbe60da300",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 350,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc850000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 13",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 351,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f1105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 37",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 352,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f1105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 17",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 353,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc850000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 354,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f1105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 355,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f1105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 17",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 356,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc850000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 357,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f1105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 358,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f1105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 17",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 359,
		"key": "0100000000000000000000000000000000000000000000000000000000000000",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 360,
		"key": "0100000000000000000000000000000000000000000000000000000000000000",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 361,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037afe03e5c298e079a79794c3a820a61614325041fc4c1be84e6169092dd4e42a01",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 362,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0eedf2a56e3f9f022ace42aa6116f576e76d0f752df8a5c879feb2288236a882d03",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 363,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 364,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 365,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 366,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 367,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 368,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 369,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 370,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 371,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 372,
		"key": "0100000000000000000000000000000000000000000000000000000000000000",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 373,
		"key": "0100000000000000000000000000000000000000000000000000000000000000",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 374,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa6245db2e055cd96ba42d2e1330ef131eefb5502759cc731c8cf787d7fd2b5a0c",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 375,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de5efb4cbf95ba7a317033c634a3bd174c2e36f6fb3a0b1d96608de7cfe4374d05",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 376,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 377,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 16",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 378,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 379,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 380,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 381,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 382,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 383,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 384,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 385,
		"key": "0100000000000000000000000000000000000000000000000000000000000000",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 386,
		"key": "0100000000000000000000000000000000000000000000000000000000000000",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 387,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7fd2c8bb23568fc2c2ab61436089349253db3045d05634c2f50f67a88ab426e709",
		"msg": "ed25519vectors 13",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 388,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf9598a4afbe7b2a777d59b9dd15ed248f498090c35ea40d691bab0cee574467807",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_component_R",
			"low_order_residue"
		]
	},
	{
		"number": 389,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 13",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 390,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 13",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 391,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 392,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 393,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 394,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 395,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 396,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 397,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 398,
		"key": "0100000000000000000000000000000000000000000000000000000000000000",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 399,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffcffc829608ae700722e38df9eb9761d200a3c86e7ef6dde961ba9cc8691cbc07",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 400,
		"key": "ef75b20e7540e3dff77404193652ba2bd13df99c1508eee1515e27ae25f28076",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd2c8bb23568fc2c2ab61436089349253db3045d05634c2f50f67a88ab426e709",
		"msg": "ed25519vectors 13",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 401,
		"key": "0100000000000000000000000000000000000000000000000000000000000080",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 14",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 402,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 403,
		"key": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 404,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 405,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 11",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 406,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 407,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 408,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "0000000000000000000000000000000000000000000000000000000000000000620683477c6eae0f5f962cfb675dc5f112275e7207521b05f57d861fb8dfa804",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 409,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "0000000000000000000000000000000000000000000000000000000000000000ab98e2715f01f57e976deea3afb2c51e93c46f5f7e054a3764fd076682034004",
		"msg": "ed25519vectors 35",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 410,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b8043872d01ef87b42d987e20d1f5604f591441bec276cb285e00c60c3854a307",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 411,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56bd79e8fd671739b78d219e13e1b578516549b9c90c988249f62d1d6a2b40ea606",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 412,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 413,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 11",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 414,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f9d21c5ce09b181f21ab1407e47442069d686bde3a0d7d250f0746e9835319f0a",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 415,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f806f1e38871587c8ed5631faf0f941c72744d2733d285b0d26eb5c7f79982b02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 416,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f620683477c6eae0f5f962cfb675dc5f112275e7207521b05f57d861fb8dfa804",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 417,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7fab98e2715f01f57e976deea3afb2c51e93c46f5f7e054a3764fd076682034004",
		"msg": "ed25519vectors 35",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 418,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "00000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 419,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "00000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 420,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 421,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 15",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 422,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "0000000000000000000000000000000000000000000000000000000000000080367dbd8f23ed46b14c764374ffd8542122d8a7b57b50e54851e702e70ca6660f",
		"msg": "ed25519vectors 17",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 423,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "00000000000000000000000000000000000000000000000000000000000000807df52a3fa4bde9595e64e46ee493b1f14777427d518e42f19b6e5a9d891a0004",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 424,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a94cf6ffd24d41962558b5ec187dd97dd62faa0e79f56e10b9588663b310ece9800",
		"msg": "ed25519vectors 16",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 425,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a94d205a702b93f4292e22186bd167770bfa3278f37a46264a735b31e757393e50f",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 426,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 427,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 428,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "edfffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff797080e0e516f4ce696c1f6508c4211246c8734fe1c740830bc07b428001007",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 429,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff881c2e6337a2fc68add023c1c3b77016cefb8304d735cca7571edde70fca1408",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 430,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff367dbd8f23ed46b14c764374ffd8542122d8a7b57b50e54851e702e70ca6660f",
		"msg": "ed25519vectors 17",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 431,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7df52a3fa4bde9595e64e46ee493b1f14777427d518e42f19b6e5a9d891a0004",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 432,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A"
		]
	},
	{
		"number": 433,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 21",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 434,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_A",
			"low_order_component_A"
		]
	},
	{
		"number": 435,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 15",
		"flags": [
			"low_order_A",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 436,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "010000000000000000000000000000000000000000000000000000000000000048435f31d933b4c6418d7d48d1223ddf2005f6ff4c6555c5850313295f5f4507",
		"msg": "ed25519vectors 25",
		"flags": [
			"low_order_R",
			"low_order_component_A"
		]
	},
	{
		"number": 437,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "0100000000000000000000000000000000000000000000000000000000000000b1daaf7afa2140df3989634cfeb1dd0704b1feab734e6032a97f08ff33650704",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 438,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350606eafe34c35b824e46e082060ca75777e699beb94810d8195d570395c050470f",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_component_A"
		]
	},
	{
		"number": 439,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a3506046f0694bed0011b9085a74ca8f2aded381a9392b8182a6475ce4067fe6a7b07",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 440,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "01000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 441,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "01000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 442,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "0100000000000000000000000000000000000000000000000000000000000080c57b6e27f4d1afcbc43510891ee7816e69732bdac3893a4c4d75a698ec76fd09",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 443,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "0100000000000000000000000000000000000000000000000000000000000080ef0227e52e05478966a93698d9ddcdf73008388227b82a9a3f4311cff41ebd01",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 444,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "010000000000000000000000000000000000000000000000000000000000008048435f31d933b4c6418d7d48d1223ddf2005f6ff4c6555c5850313295f5f4507",
		"msg": "ed25519vectors 25",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 445,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "0100000000000000000000000000000000000000000000000000000000000080b1daaf7afa2140df3989634cfeb1dd0704b1feab734e6032a97f08ff33650704",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 446,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 447,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 448,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f69d1960a5be23a7e46d14e609465c4fb82f3a486b4cf9ba3cc00d79f371a3002",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 449,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7fc5563d65a243389fca7e1e6da4223a97871ece326555c70d323ade5764f6420c",
		"msg": "ed25519vectors 14",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 450,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f48435f31d933b4c6418d7d48d1223ddf2005f6ff4c6555c5850313295f5f4507",
		"msg": "ed25519vectors 25",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 451,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7fb1daaf7afa2140df3989634cfeb1dd0704b1feab734e6032a97f08ff33650704",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 452,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 453,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 16",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 454,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff01e3f5bcb8f8809336ac1034f88ea2e77c63713da62cdd73a7f5c7550ca89208",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 455,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff161119954a491e61643a5f8ba93edccf16b52d784f8a896e5d7ba53b73c84504",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 456,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff48435f31d933b4c6418d7d48d1223ddf2005f6ff4c6555c5850313295f5f4507",
		"msg": "ed25519vectors 25",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 457,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffb1daaf7afa2140df3989634cfeb1dd0704b1feab734e6032a97f08ff33650704",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 458,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc050000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 459,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc050000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 460,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d2105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 18",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 461,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d2105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 462,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05008fba896e3cc7324a5b000fd0b903492c2ae0ae71dff7af176665ba205f3901",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 463,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc057c047ff30db9b92cac61791c64666eb0f6f11dd840d7d05b927c85f50533600e",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 464,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d21f7d88566da564c6fa777235d8864b9df8d63ee4369bcc3dbf0f421fdfe9d7309",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 465,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d21ddfb80c3b500d19d9a01010fb34cf3c00c7bc6e972e7d672e19ec969bd5b1908",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 466,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc850000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 467,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc850000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 468,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f1105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 469,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f1105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 470,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85a12ee75b7ce36d925ba81778453a97d8344a8decfdd797ee6d398a2c6820260e",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 471,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc8527d64d53fa7d8a7a56ab4dcfce52e6411ff5c5466029d30aa4159afad83f2409",
		"msg": "ed25519vectors 15",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 472,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f1123de7ac4b146e4fb8b492be4fcc69c95022bb9bba7eff6839101409bd5c5d50a",
		"msg": "ed25519vectors 14",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 473,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f116f03b9c47f23619e95184c7914c139f4a66906150bdc0635512421be6e282004",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 474,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 475,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 476,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 477,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 12",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 478,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037abeb9fdccdf6e0bb47df7c1cba374f20e53e2af651a4a69d9fa8554bcd539e50b",
		"msg": "ed25519vectors 24",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 479,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037ab6cdec95505f3fc3a3c58d45a591399c735e98c0cadaba2b61ee8b93f54d4105",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 480,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee27f196298cd2b5aec8b6545b122eb01dc4c87aff398c5152ed458036643f6a0a",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 481,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee035953cc64888fcdb0e70de5779b546a4c56b2d8f579748b4ce963887eb36705",
		"msg": "ed25519vectors 16",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 482,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 483,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 484,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 21",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 485,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 486,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fab582b7398d6916f262ad46ab12895cd197ae7a89fd94530ad4071a5d71ee870b",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 487,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa8fb5bb6032e801fdce885ff4f83560b6f56e66d3d9ad3a33eb9280190b2c670a",
		"msg": "ed25519vectors 41",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 488,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72def9a498ab4b8a710de4a88867179af3a355b93c22f79f39581f7fb196e72f3400",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 489,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72dec0096ae4cafa9e22026530dc5fd2efd3159c8b1e4bcbfee4bb18049c43a33a03",
		"msg": "ed25519vectors 41",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 490,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 12",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 491,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 492,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 493,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 494,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7ffa4a318b4063e408dec239cc0589533d04f801fe25009c1edd4c0a3b25097600",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 495,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7fd3092366957eb289ec452b34bb6783c1790a339ad5d004d9f6d435325bdc2c0a",
		"msg": "ed25519vectors 30",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 496,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf95a0b32dab50849741c136247a6109cfa76dc577cdec798e467689e43613dfc00",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 497,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf9b452dd12a8b87cd3b6a527364a5e6f99eead5739469d5d120d95e50394681605",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 498,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 499,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 11",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 500,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff1e9a40075f4b371d6575911abeb73574e1ce0ada5640bda601486ef92ef7ed0d",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 501,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff2f9a3ea3657c36624cec77ed3facee69b3eeeb32131c6b87ba0a76993d3a3a06",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 502,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "ecfffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa4a318b4063e408dec239cc0589533d04f801fe25009c1edd4c0a3b25097600",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 503,
		"key": "6718d0a3d58deaeaefa655eae3f119071deaa2cfebfd0ca28b670f879d657086",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd3092366957eb289ec452b34bb6783c1790a339ad5d004d9f6d435325bdc2c0a",
		"msg": "ed25519vectors 30",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 504,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 35",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 505,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 506,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 507,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 27",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 508,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "00000000000000000000000000000000000000000000000000000000000000006d14fb942d5ff13cef4d783375f3abef9aea3d9bcecc1a0866415dae3509d507",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 509,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "00000000000000000000000000000000000000000000000000000000000000005f5f7ffd508fc84bd8fb5d9e90a3abc24b2cbacc03b00ca42c28e7b879c5d90d",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 510,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b08a85cdeab6f57330926e2eb3891c1017863d51e99f8d8f732ee42f5433c7507",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 511,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b097c3f79437cd87c6061fb032f792cd8cf46cbb4ce390b4decb9c1bf5e70ad0b",
		"msg": "ed25519vectors 15",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 512,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 14",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 513,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 16",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 514,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7fcf5dd993590903346c6c7dbbdab784f8eb498e7074896ff06f98221870663b07",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 515,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7fca87bba547ac145cd6e813328df6df16ac5a137df85037b1c5ad2877464e840d",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 516,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f6d14fb942d5ff13cef4d783375f3abef9aea3d9bcecc1a0866415dae3509d507",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 517,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f5f5f7ffd508fc84bd8fb5d9e90a3abc24b2cbacc03b00ca42c28e7b879c5d90d",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 518,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "00000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 519,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "00000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 520,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 521,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 522,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "0000000000000000000000000000000000000000000000000000000000000080bf79045665b39525ad5ec9e9ade82bd5bc33fb049b13503dfb97aacc4ad0ae0e",
		"msg": "ed25519vectors 31",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 523,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "00000000000000000000000000000000000000000000000000000000000000806b115a2573c0a4de9f3411a96c31e1919e57bfc3e5c27faefebcdd47768b9a05",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 524,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9450b2750dbb4ca36a55e8561ae5f39235db090de324b3c60126ad1f5c1d1ae30a",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 525,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9416e0b2ca09fe5b8ba05f5f8d3b4ab0e39b79b549775a9e136fd9d6fedf3b8609",
		"msg": "ed25519vectors 12",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 526,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 527,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 528,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff9bc693ae3af4296f60ffd9afb577cabb05c2dff634a5d483e5a8c094dd068403",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 529,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff2f93b9ac542ae80e1054d7f7aff2c72cfcb5140130c37c92d4eb389c806d1509",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 530,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffbf79045665b39525ad5ec9e9ade82bd5bc33fb049b13503dfb97aacc4ad0ae0e",
		"msg": "ed25519vectors 31",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 531,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6b115a2573c0a4de9f3411a96c31e1919e57bfc3e5c27faefebcdd47768b9a05",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 532,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A"
		]
	},
	{
		"number": 533,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 534,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_A",
			"low_order_component_A"
		]
	},
	{
		"number": 535,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_A",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 536,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "010000000000000000000000000000000000000000000000000000000000000092a325759ff830a4d19c0f0cce6364311dc1e7e4bc1efaa8a54632082c05cf0f",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_component_A"
		]
	},
	{
		"number": 537,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "010000000000000000000000000000000000000000000000000000000000000048e9799d7754086f6eca319160c0d2ef1d35617c0151c719806495eb1026ac05",
		"msg": "ed25519vectors 15",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 538,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350610dba6a8317cea9cda386e05f66eabb4248890064325c1d7a567a3443ce26606",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_component_A"
		]
	},
	{
		"number": 539,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a3506ceeddc384993fffb6ea69400f4a1529ddbd0f43c6322bf53d9ec144e9127680d",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 540,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "01000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 11",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 541,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "01000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 542,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "0100000000000000000000000000000000000000000000000000000000000080cdd1248e749ceac01f109bce3ce41507e49ba65d6a42baa443d77aa63113cd0f",
		"msg": "ed25519vectors 18",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 543,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "010000000000000000000000000000000000000000000000000000000000008063e6cef756c8afdf5e0b364b01afa219cedeadb53d76c1ac58140dc8a7b37106",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 544,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "010000000000000000000000000000000000000000000000000000000000008092a325759ff830a4d19c0f0cce6364311dc1e7e4bc1efaa8a54632082c05cf0f",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 545,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "010000000000000000000000000000000000000000000000000000000000008048e9799d7754086f6eca319160c0d2ef1d35617c0151c719806495eb1026ac05",
		"msg": "ed25519vectors 15",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 546,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 547,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 548,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7fd4859f5c56a843a885bd86045dfcda0bf117f95c07c298f45664ad2f7f043106",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 549,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f4ee43c88b8d6703511f5a818428fd65e564301e1840c7ede88cf15ffac567a01",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 550,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f92a325759ff830a4d19c0f0cce6364311dc1e7e4bc1efaa8a54632082c05cf0f",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 551,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f48e9799d7754086f6eca319160c0d2ef1d35617c0151c719806495eb1026ac05",
		"msg": "ed25519vectors 15",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 552,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 553,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 554,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff2b502e797c4ce38333e3166e4e102483cd6ea5eea07bd0347a85d303c64e8b07",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 555,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff2de8d820b140fdd27706ee8011913f9542adf9641f0b2913054c4ed3c2c1be06",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 556,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff92a325759ff830a4d19c0f0cce6364311dc1e7e4bc1efaa8a54632082c05cf0f",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 557,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff48e9799d7754086f6eca319160c0d2ef1d35617c0151c719806495eb1026ac05",
		"msg": "ed25519vectors 15",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 558,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc050000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 559,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc050000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 560,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d2105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 561,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d2105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 562,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc051f8c6e2af77751e5474253bfac76ad7d3e16be0be2ff499ac17b66beafb7270e",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 563,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc0510eb9a2150d3683d1d873d608d7fb395b5968b379a0ec726bbbfbe453114fd01",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 564,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d21a66d971845a5eed7cd0432182614117196429d78205aa912a4272b134ab1740c",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 565,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d219c41f676f85cd5a5eee282112170365be8a4820e7cc4d13fdb88b1bf1f106c0b",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 566,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc850000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 567,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc850000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 568,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f1105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 569,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f1105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 570,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85c70ad24a368812eb8a9fd7bb0e300f492a0a9dbcda01df2829bca14ccf5bf10f",
		"msg": "ed25519vectors 13",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 571,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85840364150cb09a830180d30ac7907c0a858eef2a282dc539e6fd73d8a6dfad0f",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 572,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f11e35ad3a31ecbff623f4733dded01086ed68e2bdaaa53171f4b8748a26514400d",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 573,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f115f11918f318e44147d456b9e33399dd987efaa7fb06b2485e1dd2f2e0c1e4b04",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 574,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 575,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 14",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 576,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 577,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 578,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a3c8b7f81b7fccdb4c6bc85121bc77ac9b2f2b22f3241501b1521195c86f3820f",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 579,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a3b8d3ce4adae6f057f2f0b1ad9b46678443d6c79779a69254a6d112ddf435b0c",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 580,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee9e7960e86abadeb7d5e1d2492122d925685b47203130aa8e629d093c61ada90d",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 581,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee40bdefba7a8c40c0580426e32719149f9600226b48f5e27dae52b6fbbdcf5805",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 582,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 583,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 584,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 585,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 586,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa7c1368a43039c42ac2dbb604b9617df90b4d2022deee87dd75d32faf83b9b901",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 587,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03faacf8c3c342a85ff0684df2e0dbdacf3a81417755cb9f5005d22bedf43fd8c803",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 588,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de1a33a7d03bde08688f729ab233eced41ff1db67eaa08157c3a1a27a00670c701",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 589,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de54af5f2d4dd8095dbea5873cc38d78ea426817d2fd61e9de5ec5165c93e62f09",
		"msg": "ed25519vectors 11",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 590,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 591,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 12",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 592,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 13",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 593,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 594,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f9dff20f0d2acccc37928a00eb22a115047b52bcc2ab4b79b4432873d20172800",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 595,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7fd082268d79821ab3a47487f2de6f64a13c921c0ec23e89e57cd596e56f61fe0a",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 596,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf96c95b94a5ed966fe1ac4f858c49fe690ab5f2d3a4571548d943cd31375e3f20b",
		"msg": "ed25519vectors 17",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 597,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf9e8df7d32a60fca528b0606d03f8bd6e57619183cb72e99da237d97f87213d301",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 598,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 599,
		"key": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 600,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff681dcf4473cd9f43e21c3392dde10617a686de272c1014020896c9f458986202",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 601,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff35ec9aadea22201caf2fadcc24f4818d1202723a9a354e7b351094f746706300",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 602,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff9dff20f0d2acccc37928a00eb22a115047b52bcc2ab4b79b4432873d20172800",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 603,
		"key": "eca00a6a1b1f522ff2217691059915b097b73bc69bef396c36ddcd559b79e2b0",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd082268d79821ab3a47487f2de6f64a13c921c0ec23e89e57cd596e56f61fe0a",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 604,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 605,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 606,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 607,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 608,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "000000000000000000000000000000000000000000000000000000000000000031ac3ed1453a28af252ff542be21a4c0ef8b97c74940e51766b1855dbe02350d",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 609,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "0000000000000000000000000000000000000000000000000000000000000000420360a0c176d4586f28edbbcafc6e5d63d8f70954510ade434219770a86f204",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 610,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b07fe06280ad617d2c94c06e2b20983e035b2848443f7de550e5358bea36f0c03",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 611,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b945fc1469dbba776f427c47e386524dc1916bf9aad21268232b7f49b15310c06",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 612,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 613,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 614,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f8afc6fda146d589fa805b824266862baa309bb1ec071226b06ba4eaff9763c0e",
		"msg": "ed25519vectors 11",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 615,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7ffa0b5b45eb2cb24cab8a910bfbcb08641410d64790b516c583b63940d25a9504",
		"msg": "ed25519vectors 12",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 616,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f31ac3ed1453a28af252ff542be21a4c0ef8b97c74940e51766b1855dbe02350d",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 617,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f420360a0c176d4586f28edbbcafc6e5d63d8f70954510ade434219770a86f204",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 618,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "00000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 11",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 619,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "00000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 620,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 621,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 16",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 622,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "000000000000000000000000000000000000000000000000000000000000008069c99316215043ec75ea5a3d4e1357e3d0c5602366e82b24de319a520b585106",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 623,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "0000000000000000000000000000000000000000000000000000000000000080c9a28760bfdbaf4f028fe8d8ecb8c34db3b7c0c71d24e26734b2c841e8fac207",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 624,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a94e8b6da4567e7bfba5829e7a1197f8062462342ba28f67b2cbcf14a28af6e970a",
		"msg": "ed25519vectors 12",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 625,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a948f8fa9222af828b040bcd86187c66d23faa97d1a51be5d5126bfd78688897b00",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 626,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 627,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 628,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff09b548baa15c220f0c456454bc0b3501750239f69aad45100932d36f0e9f3200",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 629,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa537bd259769d9113a834f55e1f42b160fad630daf58c8f9c7e80d8e5daa1302",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 630,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff69c99316215043ec75ea5a3d4e1357e3d0c5602366e82b24de319a520b585106",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 631,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc9a28760bfdbaf4f028fe8d8ecb8c34db3b7c0c71d24e26734b2c841e8fac207",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 632,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A"
		]
	},
	{
		"number": 633,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 12",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 634,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_A",
			"low_order_component_A"
		]
	},
	{
		"number": 635,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_A",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 636,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "0100000000000000000000000000000000000000000000000000000000000000c46cc62cf59c79dc96ec34243a10a14750194479376a49cc84651a23a488ee05",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_A"
		]
	},
	{
		"number": 637,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "01000000000000000000000000000000000000000000000000000000000000007cbd8c879c5bf2335aa640bc7becf0547146c5becaae73560e9e80c036f2d502",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 638,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a3506104c833a1253a48cd66e54cab38543281b75e53c8bccb6a1dccd45eb251f1307",
		"msg": "ed25519vectors 14",
		"flags": [
			"low_order_component_A"
		]
	},
	{
		"number": 639,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a3506f3ae4bb3afa88c3285c8b1241cf6df8316bb4b35e71346248582cd627121ad02",
		"msg": "ed25519vectors 13",
		"flags": [
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 640,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "01000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 25",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 641,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "01000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 642,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "01000000000000000000000000000000000000000000000000000000000000809dfcac23338d3f13dc0ff5de059db61934190741f1f00836d7b735975892e002",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 643,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "0100000000000000000000000000000000000000000000000000000000000080230333f78f5b3eacd737fb60742c5c041e1ea56410ac40cc56b5b40968cc1209",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 644,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "0100000000000000000000000000000000000000000000000000000000000080c46cc62cf59c79dc96ec34243a10a14750194479376a49cc84651a23a488ee05",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 645,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "01000000000000000000000000000000000000000000000000000000000000807cbd8c879c5bf2335aa640bc7becf0547146c5becaae73560e9e80c036f2d502",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 646,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 647,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 13",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 648,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f3a63dbb9c17d6ca41a6e1ce55b0a99bf8025eb49cc4640b3de424cb665fba20c",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 649,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f5b028c7e15842ffa8f6a8effc6f3a447419f4228cbfba57ebdf522573ed38f08",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 650,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7fc46cc62cf59c79dc96ec34243a10a14750194479376a49cc84651a23a488ee05",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 651,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f7cbd8c879c5bf2335aa640bc7becf0547146c5becaae73560e9e80c036f2d502",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 652,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 653,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 16",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 654,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff92d85e93707e2f67157e9e4b3e289b823be4ccec80abd861efb9ba2e4c32510e",
		"msg": "ed25519vectors 16",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 655,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffaced0fbc977b44ec36d82176d929c9878bc6a90dbabdac84f071fa6daf97050a",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 656,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc46cc62cf59c79dc96ec34243a10a14750194479376a49cc84651a23a488ee05",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 657,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7cbd8c879c5bf2335aa640bc7becf0547146c5becaae73560e9e80c036f2d502",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 658,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc050000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 659,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc050000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 660,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d2105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 661,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d2105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 662,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc053fe4d58f46c5232b61f8e315b21a99832e7edb9875c9c2cfd0d77626215b580a",
		"msg": "ed25519vectors 15",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 663,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc053d8acb56c081e29da3329a1f78ceff08c5f925cde1c64690dbf91d5260bcd50b",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 664,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d21ea7f1306764d7e910babd66ac6b9ed1b7db6c0381fbefe5d98b70a2c8fe97407",
		"msg": "ed25519vectors 13",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 665,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d2110ac576cd0481197776c0121a40caf560b3fc593fa0982e1e274007955fe530a",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 666,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc850000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 667,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc850000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 668,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f1105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 669,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f1105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 17",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 670,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc858c6564558b985cc383035bfb3572083c323717acb6c014db50b4a539a1e8900a",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 671,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc851feb0bd9cb3ea94d28ecce323f06901a96b3ea2f9e5b885aafa13d571ae4f70b",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 672,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f11fa299c89564128d4fe83c4758eb9c28990407191109ad31f0ca2b1548da63404",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 673,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f119a4fca0c7482f8d9f37923d8702d104d07b549e08225193b2818a6380e9ab40f",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 674,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 675,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 676,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 677,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 13",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 678,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037ae37e2cac72091af68b51b43b40f15e499c41621707e1fb30cb17362333cbba09",
		"msg": "ed25519vectors 12",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 679,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a1e7d219f4540986f8904c6a16a2b74967c013b9ff1a99e6b9fdbb4835c19d70d",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 680,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee9101d1fb7d624768ec8ea7e31aca21042f3495ad5e48fdd151423f5c5f216c04",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 681,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0eef282a24a316fe5449ea437d32470df2c968f2f27688653d1e026ad3d3bfb530c",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 682,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 683,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 684,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 11",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 685,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 16",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 686,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fabfad7e6fdf808e46ddaaa65ad1e5fc4751473c239f63c49c2e017d7209058900",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 687,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa750d023533f655c6f32c38992a8a30f9f20c59bda30682a7f5f692764724e604",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 688,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72def2c459284b532cb7a74779ced5cc538521b7e466561e8abdcda7f64238404200",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 689,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de55ba20be6f48cddf9d90aef69958f1c53baa96226c0d788f25b13754d1a33207",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 690,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 691,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 692,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 693,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 694,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7ff14a0b307122c632de6f34f1107225f88b1e21893f4d6719ddefd90f44b11f05",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 695,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f517fa7eb17544d51490f2417a13f0d85919dd5998cc3c465b8c3614f4651d20d",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 696,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf9a05c2ebdd26e6d36b0d39a6dc8cad91ac0934700d6b796b6d7499d6eb81f1301",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 697,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf9e97e9b1f47281ed688a320de5912618b62c9dac64d5971e2aab2d487038cc90b",
		"msg": "ed25519vectors 18",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 698,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 699,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 700,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff248f13d1b0c2af4782513a8c33da39dd7cf9caf4ee3c85e58dbcbaec928aee0b",
		"msg": "ed25519vectors 20",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 701,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff10e1e32f76302558b064ad4873824195afc9fbdd1a6aa20a8dd05cd54f708f00",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 702,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "ecfffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff14a0b307122c632de6f34f1107225f88b1e21893f4d6719ddefd90f44b11f05",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 703,
		"key": "015ff595e4e0add00dde896efa66ea4f6848c4396410c693c92232aa64861d4f",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff517fa7eb17544d51490f2417a13f0d85919dd5998cc3c465b8c3614f4651d20d",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 704,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 14",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 705,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 30",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 706,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 12",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 707,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 708,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "0000000000000000000000000000000000000000000000000000000000000000ff3d622b55aad17c5c7aad97a56a7ada9eda2a92a14a4c73e349d8da4f64c70a",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 709,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "00000000000000000000000000000000000000000000000000000000000000000734b8823047a23fb776be86db7b3f4bb1c9e49bc29b352c760002bdc8e54d02",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 710,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b55a1b09e210288e47a4796def776679cdec5a64bd2bb9bc69932c3bd47b96109",
		"msg": "ed25519vectors 11",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 711,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b074fc0dd361fa65cf275443e324c504017fa8e80bba2a1c5ad46f116c3dac901",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 712,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 713,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 714,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7fa6f79db3f81348fd22224e50a25621f810d038de2060250f8a4937f3b4cc5208",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 715,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f59c48433c25fcb5f435c328aff1bad6eaf88cc0d7586a723ba3dc921f5009f0a",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 716,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7fff3d622b55aad17c5c7aad97a56a7ada9eda2a92a14a4c73e349d8da4f64c70a",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 717,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0734b8823047a23fb776be86db7b3f4bb1c9e49bc29b352c760002bdc8e54d02",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 718,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "00000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 719,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "00000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 720,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 721,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 722,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "0000000000000000000000000000000000000000000000000000000000000080216c38d9eae130fee53569d8a1750b2f2ef49185cddd7cf43379e84a3ef50d04",
		"msg": "ed25519vectors 12",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 723,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "000000000000000000000000000000000000000000000000000000000000008026d690d9305a0d0a44ebb4e82d11b7fb6e0850fde3b27cdf662dfd300393eb01",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 724,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9452834a44fb880f6b87a7e173a2856a90f987bef39f1d95b066a528099af13302",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 725,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a94a69145f508ef4ce1a0fd25025d82516e06e492ef088de45da784f8a863ebe401",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 726,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 727,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 728,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff56668e1aa7f6b00250315677063e6f347c869cc31c7a9f31ed42d02e1bdd9e0d",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 729,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff368ce93ef4a864890115ed9cb1341fa32fa0996911fe07c1496e8c9a3df43a08",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 730,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff216c38d9eae130fee53569d8a1750b2f2ef49185cddd7cf43379e84a3ef50d04",
		"msg": "ed25519vectors 12",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 731,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff26d690d9305a0d0a44ebb4e82d11b7fb6e0850fde3b27cdf662dfd300393eb01",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 732,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A"
		]
	},
	{
		"number": 733,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 734,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_A",
			"low_order_component_A"
		]
	},
	{
		"number": 735,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 12",
		"flags": [
			"low_order_A",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 736,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "0100000000000000000000000000000000000000000000000000000000000000cdf25d18d1d94aa074c5277cafc39c51286021e21678ccc919bd6f326cfcb709",
		"msg": "ed25519vectors 11",
		"flags": [
			"low_order_R",
			"low_order_component_A"
		]
	},
	{
		"number": 737,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "01000000000000000000000000000000000000000000000000000000000000004071f5e71c483f0886dd678575b498d03177d51ad542edf113b6214b93c79b0b",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 738,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a3506bc1f0d2c7b653d5aaf07a71c130d8aff5bef8653d6984f2b223081ac24a62005",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_component_A"
		]
	},
	{
		"number": 739,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a3506134ff3e6e5e3fce6d8f46295871735f1700c2674d3892a7f06f055533336a002",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 740,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "01000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 741,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "01000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 13",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 742,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "010000000000000000000000000000000000000000000000000000000000008040ab130e002935091a7eac0f1476b9b5e60411ead58a3c0e95765c234752f702",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 743,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "0100000000000000000000000000000000000000000000000000000000000080f28144c16a59ecedbbe82aa3481cf42279d82c6c669beb6c59622e94bcc03808",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 744,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "0100000000000000000000000000000000000000000000000000000000000080cdf25d18d1d94aa074c5277cafc39c51286021e21678ccc919bd6f326cfcb709",
		"msg": "ed25519vectors 11",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 745,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "01000000000000000000000000000000000000000000000000000000000000804071f5e71c483f0886dd678575b498d03177d51ad542edf113b6214b93c79b0b",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 746,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 747,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 25",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 748,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f4524c928a583ec7cbddab4765dbdaf532c7fd278a5c4fd933e9c4c01c2f58302",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 749,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f335ec19261efdcd25bbe64dde6dcc7815b237bf53ffedc45d44351d9690e0e0b",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 750,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7fcdf25d18d1d94aa074c5277cafc39c51286021e21678ccc919bd6f326cfcb709",
		"msg": "ed25519vectors 11",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 751,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f4071f5e71c483f0886dd678575b498d03177d51ad542edf113b6214b93c79b0b",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 752,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 753,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 754,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff69a153a2d8922b9fcfbc06a425e6a50f28dfe3799f91cb5f983b16eadf930405",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 755,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffb61ad06789857c7a7c9025df6b6321483f533c775ababa02b57ea2508507340c",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 756,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffcdf25d18d1d94aa074c5277cafc39c51286021e21678ccc919bd6f326cfcb709",
		"msg": "ed25519vectors 11",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 757,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff4071f5e71c483f0886dd678575b498d03177d51ad542edf113b6214b93c79b0b",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 758,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc050000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 759,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc050000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 760,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d2105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 761,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d2105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 762,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05dd5e46f50f59e854fda2b2ae82c6f0c64da07146d4b7d388a9c0acec0387de06",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 763,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc057a8afc08cd56ba356afe59be2d03de9d4d0f44cd72c00f85ddb1e844275b9601",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 764,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d21e25115117021871ba256c845f462422e380099235eb48e8fc898ef6b49ee5a0c",
		"msg": "ed25519vectors 21",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 765,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d214f406acc5ef89838ac824d84529e2557b738d678e3b9070634232af8e8a16d02",
		"msg": "ed25519vectors 15",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 766,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc850000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 767,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc850000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 768,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f1105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 13",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 769,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f1105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 770,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85e12d9510d58203c89c7d06a611815a2b390a6f363885b90da266c85135bc5c05",
		"msg": "ed25519vectors 17",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 771,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc850bbeccb5223743bd17f3a73dac85074bce77ce102d031b2e2a897a8b66ec650b",
		"msg": "ed25519vectors 39",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 772,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f11701a598b9a02ae60505dd0c2938a1a0c2d6ffd4676cfb49125b19e9cb358da06",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 773,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f111a1ba75dc71a1c3275e4dd3115b1638f67d16f1440d16dcc2c0188bbb17d6206",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 774,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 775,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 776,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 777,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 778,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a1d48955627f685fca00a5d62886fa4cae966af752251034904358448891d9000",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 779,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a19775c38558dafa093e9e2492933bcd4dd13656d43b75c7d13b450a986cc360d",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 780,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee91cfdeb875d86bf70f14733791c85add6b98dc813842c1338c48ea3e53e4aa0f",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 781,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee3939ad4e3797f4d869b51cb36f6b937987df797d46b851c6abc76ef707c60503",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 782,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 783,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 784,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 785,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 786,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03faf51deee4ffb00d126b97ff2d901b8e7de8bfb8a96e1dfabf4949a30b1c28ec01",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 787,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa4b4f0d7171f609289d3bbb7cbcefb424a13b468e848d58fddbb73429040c7906",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 788,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de1bdf452e3274bda9648c0e27ac7139f6c99c7ff2e96637afe541ce414e378b05",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 789,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de83458650b497c3f7226cb93e9324df567b1adda39378e844230453b95aa8c801",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 790,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 791,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 792,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 793,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 794,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f96d76d748c70430e986190b547bef03c36d3e53d4834f5c60b23d392695bbe06",
		"msg": "ed25519vectors 22",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 795,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7ff899fcb87a689f9a17bb3d7a54ab6bbd060f3f3061502f1fa6a1fc2a8eee0603",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 796,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf9e9bcd5643add13de8c05d9e630359815212df872304bef491f58f867bd542709",
		"msg": "ed25519vectors 15",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 797,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf904864848c5ff5361609e941b2136012ac88139a34707e12cadf6645dff0b1008",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 798,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 799,
		"key": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 800,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff77de70f3611eb29ee726ca74d20267c184a35f0fbc4261458eaad86f545d5b03",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 801,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff5c1a58a253fdd422102558a41077d95055a4f988bb5f475006a41a79d3a2ba01",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 802,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff96d76d748c70430e986190b547bef03c36d3e53d4834f5c60b23d392695bbe06",
		"msg": "ed25519vectors 22",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 803,
		"key": "86e72f5c2a7215151059aa151c0ee6f8e2155d301402f35d7498f078629a8f79",
		"sig": "ecfffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff899fcb87a689f9a17bb3d7a54ab6bbd060f3f3061502f1fa6a1fc2a8eee0603",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 804,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 805,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 806,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "0000000000000000000000000000000000000000000000000000000000000000819aa7c9081f2e43b7524fdd27ef578f48dd9f02371b31f8013bd0c5321c660f",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 807,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b9dbda34f2ac60ee9d893b9b16f898617e81347886067f49d79d37740bb42a80b",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 808,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 809,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 810,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "36684ea91032ba5b1dbab2d02f4debc74c3327f2b3802e2e4d371aa42b12b56b05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 811,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 812,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f1630a351114792030905842b0d440c30c3c3c08f8e275cc32718756675d10f06",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 813,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f819aa7c9081f2e43b7524fdd27ef578f48dd9f02371b31f8013bd0c5321c660f",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 814,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 815,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "00000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 19",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 816,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 817,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "0000000000000000000000000000000000000000000000000000000000000080008207ec4d9a9b8aaeee217ecb5d87a958de17beb51faec53236e7f7e07e6c05",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 818,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9467f05af9575f4d1a13f23973e28c591c4944c7dec5e4178c71a88110cc175006",
		"msg": "ed25519vectors 12",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 819,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "00000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 820,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 821,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "b797b156efcd45a4e2454d2fd0b21438b3ccd80d4c7fd1d1b2c8e55bd4ed4a9405ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 822,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 823,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff48ed3504f9d9a85115643ab1fefe191b70c39dc708708236227941792dc8c502",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 824,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff008207ec4d9a9b8aaeee217ecb5d87a958de17beb51faec53236e7f7e07e6c05",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 825,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 826,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 14",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A"
		]
	},
	{
		"number": 827,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 828,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 14",
		"flags": [
			"low_order_A",
			"low_order_component_A"
		]
	},
	{
		"number": 829,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_A",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 830,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "0100000000000000000000000000000000000000000000000000000000000000ce34fe4edd707095877049d405f52b52a726b4cbef9b8a1f950340d521fe110d",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_component_A"
		]
	},
	{
		"number": 831,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "01000000000000000000000000000000000000000000000000000000000000007798d1693338d7c46e61a3aae05bd23a89fdf7b62b83efdd062dd19a39d8d505",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 832,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a3506ccbd210592f2a2b70a9c9ba91f97d642a2e51b9a67ec788188039228a24e0e09",
		"msg": "ed25519vectors 23",
		"flags": [
			"low_order_component_A"
		]
	},
	{
		"number": 833,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350660f862046e40dcc3af08e1b97b6cd10ee44158cbccab65668862e844ace00500",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 834,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 12",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_A"
		]
	},
	{
		"number": 835,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 836,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_A",
			"low_order_component_A",
			"non_canonical_A"
		]
	},
	{
		"number": 837,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 838,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 14",
		"flags": [
			"low_order_A",
			"low_order_component_A",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 839,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "b62cf890de42c413b11b1411c9f01f1c4d77aa87ef182258d1251f69af2a350605ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 840,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "01000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 24",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 841,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "01000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 16",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 842,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "010000000000000000000000000000000000000000000000000000000000008041bd7afd3d12b42f00f9ac87804fceeea002eb2800665b0fe8acd0cf53ee3207",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 843,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "0100000000000000000000000000000000000000000000000000000000000080739deeff2a8c311e6172a2e9d05f6d8a048df123aa27e1015bda974e6b32b306",
		"msg": "ed25519vectors 21",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 844,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "0100000000000000000000000000000000000000000000000000000000000080ce34fe4edd707095877049d405f52b52a726b4cbef9b8a1f950340d521fe110d",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 845,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "01000000000000000000000000000000000000000000000000000000000000807798d1693338d7c46e61a3aae05bd23a89fdf7b62b83efdd062dd19a39d8d505",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 846,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 847,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 848,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f736cd390d6a2cb3d3a40bbbe09c87fa3caced72cdd853bfbf047adf1dec92207",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 849,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7fbe39026eed2d5f2b23510d25bc1a7bface53b1d7b949facee0c7f6d1121bbe02",
		"msg": "ed25519vectors 23",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 850,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7fce34fe4edd707095877049d405f52b52a726b4cbef9b8a1f950340d521fe110d",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 851,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f7798d1693338d7c46e61a3aae05bd23a89fdf7b62b83efdd062dd19a39d8d505",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 852,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 853,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 854,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8a4fe32e72294d34ff5060efff2141687dd52117f36311af924b73638f7bc604",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 855,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff2e6065b5a7c4aa919800747605e99800c074041d01eecca3ac39b78ef00da906",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 856,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffce34fe4edd707095877049d405f52b52a726b4cbef9b8a1f950340d521fe110d",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 857,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7798d1693338d7c46e61a3aae05bd23a89fdf7b62b83efdd062dd19a39d8d505",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_A",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 858,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "01000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 859,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "01000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 860,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 861,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 862,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 863,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 864,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc050000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 865,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d2105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 866,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05dcf76895217bf0dacec839953c960ee6d7840b9fa8ec66377df8a2e2db722305",
		"msg": "ed25519vectors 10",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 867,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d211566a1ad3f92ada58707e452dcd290efc6a1951aaefe43be3b4663e38c3ac002",
		"msg": "ed25519vectors 14",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 868,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc050000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 14",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 869,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d2105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 870,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "5ae36d433a7bd0efb150b6b04610d1986e3044c46b6ad69bae17aaf76b608d2105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 871,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc850000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 872,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f1105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 14",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 873,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc8507e25ac429f3fda828fd20bbe35e9d834875b64098f05e40d1bbe63f20b9c50d",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 874,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f113fc2c91b53d127bfc4fb6910467e737fc5a6463963ca4df83d0c82a419299e06",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 875,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc850000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 876,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f1105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 877,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "fa9dde274f4820efb19a890f8ba2d8791710a4303ceef4aedf9dddc4e81a1f1105ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 14",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 878,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 879,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 13",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 880,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a3540a57b2ba00d60510ca5174b63f5ad6289f50241887ec114583b643dfe6003",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 881,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0eef081c939b0fb2f42749cd392be91b90b20875f6a7abd4019a470299569f16f01",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 882,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 9",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 883,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 884,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "f36121d8b0b7df104e6576f0745d2786e8ef5bcfc3110b512062223b17e5e0ee05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 13",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 885,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 11",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 886,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 887,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa8c7fe7a9b40ddb9617a6ca678729b53ad7c9916531c829288e416e56fbb74809",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 888,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de73da5e09d360debd54177128d8b403f3d8cdd80ec83cd60b138b515d89d5cb0a",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 889,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 890,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 19",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 891,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "931c92bcc5842f104eaf494fb9ef2e6791cfbb3b9495296451e85508949f72de05ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 892,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 893,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 894,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 14",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 895,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 896,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f63ec463daf4a74749995e1bca07d169051630e9cf36860b86536f5c7f6e87405",
		"msg": "ed25519vectors 36",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 897,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f8fcc9e160cf71f1343cf2a6cc8d51cae1a9dc2e3debc99d97ec1190782406e05",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 898,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf92adec0cbdc3718697e370f88b291cbe1965f51921474e0fe35973dbc471c3e01",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue"
		]
	},
	{
		"number": 899,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf9327f51a83cacea96d1f3fc6be0e2682f22ce35400ccb707aa30a7321ed6dff05",
		"msg": "ed25519vectors 3",
		"flags": [
			"low_order_component_R",
			"low_order_component_A"
		]
	},
	{
		"number": 900,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 901,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_A"
		]
	},
	{
		"number": 902,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A"
		]
	},
	{
		"number": 903,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 4",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_A"
		]
	},
	{
		"number": 904,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 14",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 905,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "37d3076f21bd3bec4ee4ebee360fe0e3b288557810e7dda72edae09650d5caf905ba9a796274d80437afa36f1236563f2f3b0aa84cecddc3d20914615ba4fe02",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"reencoded_k"
		]
	},
	{
		"number": 906,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 907,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 5",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 908,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0d5f3d4d7d4fd1b055ea05193ec32458d796b69aca128d34d5e4dbaec8a86e0c",
		"msg": "ed25519vectors 2",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R"
		]
	},
	{
		"number": 909,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff472e9141da7cc7352acd9c8688b89b9c2ab873aa6c4270e9c9830051f861860f",
		"msg": "ed25519vectors 6",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R"
		]
	},
	{
		"number": 910,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff63ec463daf4a74749995e1bca07d169051630e9cf36860b86536f5c7f6e87405",
		"msg": "ed25519vectors 36",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 911,
		"key": "fe894df18abf1c20088bfbe6c9ad45d42ec20663eaf7111eaea1d851da0d7f89",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8fcc9e160cf71f1343cf2a6cc8d51cae1a9dc2e3debc99d97ec1190782406e05",
		"msg": "ed25519vectors 7",
		"flags": [
			"low_order_R",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_R",
			"reencoded_k"
		]
	},
	{
		"number": 912,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 1",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"low_order_residue",
			"non_canonical_A",
			"non_canonical_R"
		]
	},
	{
		"number": 913,
		"key": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
		"sig": "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000",
		"msg": "ed25519vectors 8",
		"flags": [
			"low_order_R",
			"low_order_A",
			"low_order_component_R",
			"low_order_component_A",
			"non_canonical_A",
			"non_canonical_R"
		]
	}
]
"#;