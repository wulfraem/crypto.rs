[
    // https://github.com/trezor/python-mnemonic/blob/master/vectors.json
    // mangled using utils/test_vectors/bip39/trezor.sh
    TestVector {
        wordlist: wordlist::ENGLISH,
        entropy: "00000000000000000000000000000000",
        mnemonic: "6162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e2061626f7574",
        passphrase: "5452455a4f52",
        seed: "c55257c360c07c72029aebc1b53c05ed0362ada38ead3e3e9efa3708e53495531f09a6987599d18264c1e1c92f2cf141630c7a3c4ab7c81b2f001698e7463b04",
    },
    TestVector {
        wordlist: wordlist::ENGLISH,
        entropy: "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
        mnemonic: "6c6567616c2077696e6e6572207468616e6b20796561722077617665207361757361676520776f7274682075736566756c206c6567616c2077696e6e6572207468616e6b2079656c6c6f77",
        passphrase: "5452455a4f52",
        seed: "2e8905819b8723fe2c1d161860e5ee1830318dbf49a83bd451cfb8440c28bd6fa457fe1296106559a3c80937a1c1069be3a3a5bd381ee6260e8d9739fce1f607",
    },
    TestVector {
        wordlist: wordlist::ENGLISH,
        entropy: "80808080808080808080808080808080",
        mnemonic: "6c65747465722061647669636520636167652061627375726420616d6f756e7420646f63746f722061636f75737469632061766f6964206c65747465722061647669636520636167652061626f7665",
        passphrase: "5452455a4f52",
        seed: "d71de856f81a8acc65e6fc851a38d4d7ec216fd0796d0a6827a3ad6ed5511a30fa280f12eb2e47ed2ac03b5c462a0358d18d69fe4f985ec81778c1b370b652a8",
    },
    TestVector {
        wordlist: wordlist::ENGLISH,
        entropy: "ffffffffffffffffffffffffffffffff",
        mnemonic: "7a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f2077726f6e67",
        passphrase: "5452455a4f52",
        seed: "ac27495480225222079d7be181583751e86f571027b0497b5b5d11218e0a8a13332572917f0f8e5a589620c6f15b11c61dee327651a14c34e18231052e48c069",
    },
    TestVector {
        wordlist: wordlist::ENGLISH,
        entropy: "000000000000000000000000000000000000000000000000",
        mnemonic: "6162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206167656e74",
        passphrase: "5452455a4f52",
        seed: "035895f2f481b1b0f01fcf8c289c794660b289981a78f8106447707fdd9666ca06da5a9a565181599b79f53b844d8a71dd9f439c52a3d7b3e8a79c906ac845fa",
    },
    TestVector {
        wordlist: wordlist::ENGLISH,
        entropy: "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
        mnemonic: "6c6567616c2077696e6e6572207468616e6b20796561722077617665207361757361676520776f7274682075736566756c206c6567616c2077696e6e6572207468616e6b20796561722077617665207361757361676520776f7274682075736566756c206c6567616c2077696c6c",
        passphrase: "5452455a4f52",
        seed: "f2b94508732bcbacbcc020faefecfc89feafa6649a5491b8c952cede496c214a0c7b3c392d168748f2d4a612bada0753b52a1c7ac53c1e93abd5c6320b9e95dd",
    },
    TestVector {
        wordlist: wordlist::ENGLISH,
        entropy: "808080808080808080808080808080808080808080808080",
        mnemonic: "6c65747465722061647669636520636167652061627375726420616d6f756e7420646f63746f722061636f75737469632061766f6964206c65747465722061647669636520636167652061627375726420616d6f756e7420646f63746f722061636f75737469632061766f6964206c657474657220616c77617973",
        passphrase: "5452455a4f52",
        seed: "107d7c02a5aa6f38c58083ff74f04c607c2d2c0ecc55501dadd72d025b751bc27fe913ffb796f841c49b1d33b610cf0e91d3aa239027f5e99fe4ce9e5088cd65",
    },
    TestVector {
        wordlist: wordlist::ENGLISH,
        entropy: "ffffffffffffffffffffffffffffffffffffffffffffffff",
        mnemonic: "7a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207768656e",
        passphrase: "5452455a4f52",
        seed: "0cd6e5d827bb62eb8fc1e262254223817fd068a74b5b449cc2f667c3f1f985a76379b43348d952e2265b4cd129090758b3e3c2c49103b5051aac2eaeb890a528",
    },
    TestVector {
        wordlist: wordlist::ENGLISH,
        entropy: "0000000000000000000000000000000000000000000000000000000000000000",
        mnemonic: "6162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e206162616e646f6e20617274",
        passphrase: "5452455a4f52",
        seed: "bda85446c68413707090a52022edd26a1c9462295029f2e60cd7c4f2bbd3097170af7a4d73245cafa9c3cca8d561a7c3de6f5d4a10be8ed2a5e608d68f92fcc8",
    },
    TestVector {
        wordlist: wordlist::ENGLISH,
        entropy: "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
        mnemonic: "6c6567616c2077696e6e6572207468616e6b20796561722077617665207361757361676520776f7274682075736566756c206c6567616c2077696e6e6572207468616e6b20796561722077617665207361757361676520776f7274682075736566756c206c6567616c2077696e6e6572207468616e6b20796561722077617665207361757361676520776f727468207469746c65",
        passphrase: "5452455a4f52",
        seed: "bc09fca1804f7e69da93c2f2028eb238c227f2e9dda30cd63699232578480a4021b146ad717fbb7e451ce9eb835f43620bf5c514db0f8add49f5d121449d3e87",
    },
    TestVector {
        wordlist: wordlist::ENGLISH,
        entropy: "8080808080808080808080808080808080808080808080808080808080808080",
        mnemonic: "6c65747465722061647669636520636167652061627375726420616d6f756e7420646f63746f722061636f75737469632061766f6964206c65747465722061647669636520636167652061627375726420616d6f756e7420646f63746f722061636f75737469632061766f6964206c65747465722061647669636520636167652061627375726420616d6f756e7420646f63746f722061636f757374696320626c657373",
        passphrase: "5452455a4f52",
        seed: "c0c519bd0e91a2ed54357d9d1ebef6f5af218a153624cf4f2da911a0ed8f7a09e2ef61af0aca007096df430022f7a2b6fb91661a9589097069720d015e4e982f",
    },
    TestVector {
        wordlist: wordlist::ENGLISH,
        entropy: "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        mnemonic: "7a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f207a6f6f20766f7465",
        passphrase: "5452455a4f52",
        seed: "dd48c104698c30cfe2b6142103248622fb7bb0ff692eebb00089b32d22484e1613912f0a5b694407be899ffd31ed3992c456cdf60f5d4564b8ba3f05a69890ad",
    },
    TestVector {
        wordlist: wordlist::ENGLISH,
        entropy: "9e885d952ad362caeb4efe34a8e91bd2",
        mnemonic: "6f7a6f6e65206472696c6c2067726162206669626572206375727461696e2067726163652070756464696e67207468616e6b2063727569736520656c646572206569676874207069636e6963",
        passphrase: "5452455a4f52",
        seed: "274ddc525802f7c828d8ef7ddbcdc5304e87ac3535913611fbbfa986d0c9e5476c91689f9c8a54fd55bd38606aa6a8595ad213d4c9c9f9aca3fb217069a41028",
    },
    TestVector {
        wordlist: wordlist::ENGLISH,
        entropy: "6610b25967cdcca9d59875f5cb50b0ea75433311869e930b",
        mnemonic: "67726176697479206d616368696e65206e6f72746820736f72742073797374656d2066656d616c652066696c74657220617474697475646520766f6c756d6520666f6c6420636c756220737461792066656174757265206f66666963652065636f6c6f677920737461626c65206e6172726f7720666f67",
        passphrase: "5452455a4f52",
        seed: "628c3827a8823298ee685db84f55caa34b5cc195a778e52d45f59bcf75aba68e4d7590e101dc414bc1bbd5737666fbbef35d1f1903953b66624f910feef245ac",
    },
    TestVector {
        wordlist: wordlist::ENGLISH,
        entropy: "68a79eaca2324873eacc50cb9c6eca8cc68ea5d936f98787c60c7ebc74e6ce7c",
        mnemonic: "68616d73746572206469616772616d20707269766174652064757463682063617573652064656c61792070726976617465206d65617420736c69646520746f64646c65722072617a6f7220626f6f6b2068617070792066616e637920676f7370656c2074656e6e6973206d61706c652064696c656d6d61206c6f616e20776f726420736872756720696e666c6963742064656c6179206c656e677468",
        passphrase: "5452455a4f52",
        seed: "64c87cde7e12ecf6704ab95bb1408bef047c22db4cc7491c4271d170a1b213d20b385bc1588d9c7b38f1b39d415665b8a9030c9ec653d75e65f847d8fc1fc440",
    },
    TestVector {
        wordlist: wordlist::ENGLISH,
        entropy: "c0ba5a8e914111210f2bd131f3d5e08d",
        mnemonic: "736368656d652073706f742070686f746f20636172642062616279206d6f756e7461696e20646576696365206b69636b20637261646c652070616374206a6f696e20626f72726f77",
        passphrase: "5452455a4f52",
        seed: "ea725895aaae8d4c1cf682c1bfd2d358d52ed9f0f0591131b559e2724bb234fca05aa9c02c57407e04ee9dc3b454aa63fbff483a8b11de949624b9f1831a9612",
    },
    TestVector {
        wordlist: wordlist::ENGLISH,
        entropy: "6d9be1ee6ebd27a258115aad99b7317b9c8d28b6d76431c3",
        mnemonic: "686f726e2074656e616e74206b6e65652074616c656e742073706f6e736f72207370656c6c206761746520636c69702070756c736520736f617020736c757368207761726d2073696c766572206e6570686577207377617020756e636c6520637261636b206272617665",
        passphrase: "5452455a4f52",
        seed: "fd579828af3da1d32544ce4db5c73d53fc8acc4ddb1e3b251a31179cdb71e853c56d2fcb11aed39898ce6c34b10b5382772db8796e52837b54468aeb312cfc3d",
    },
    TestVector {
        wordlist: wordlist::ENGLISH,
        entropy: "9f6a2878b2520799a44ef18bc7df394e7061a224d2c33cd015b157d746869863",
        mnemonic: "70616e64612065796562726f772062756c6c657420676f72696c6c612063616c6c20736d6f6b65206d756666696e207461737465206d65736820646973636f76657220736f6674206f73747269636820616c636f686f6c207370656564206e6174696f6e20666c617368206465766f7465206c6576656c20686f62627920717569636b20696e6e65722064726976652067686f737420696e73696465",
        passphrase: "5452455a4f52",
        seed: "72be8e052fc4919d2adf28d5306b5474b0069df35b02303de8c1729c9538dbb6fc2d731d5f832193cd9fb6aeecbc469594a70e3dd50811b5067f3b88b28c3e8d",
    },
    TestVector {
        wordlist: wordlist::ENGLISH,
        entropy: "23db8160a31d3e0dca3688ed941adbf3",
        mnemonic: "636174207377696e6720666c61672065636f6e6f6d79207374616469756d20616c6f6e6520636875726e20737065656420756e69717565207061746368207265706f727420747261696e",
        passphrase: "5452455a4f52",
        seed: "deb5f45449e615feff5640f2e49f933ff51895de3b4381832b3139941c57b59205a42480c52175b6efcffaa58a2503887c1e8b363a707256bdd2b587b46541f5",
    },
    TestVector {
        wordlist: wordlist::ENGLISH,
        entropy: "8197a4a47f0425faeaa69deebc05ca29c0a5b5cc76ceacc0",
        mnemonic: "6c696768742072756c652063696e6e616d6f6e2077726170206472617374696320776f726420707269646520737175697272656c2075706772616465207468656e20696e636f6d6520666174616c206170617274207375737461696e20637261636b20737570706c792070726f756420616363657373",
        passphrase: "5452455a4f52",
        seed: "4cbdff1ca2db800fd61cae72a57475fdc6bab03e441fd63f96dabd1f183ef5b782925f00105f318309a7e9c3ea6967c7801e46c8a58082674c860a37b93eda02",
    },
    TestVector {
        wordlist: wordlist::ENGLISH,
        entropy: "066dca1a2bb7e8a1db2832148ce9933eea0f3ac9548d793112d9a95c9407efad",
        mnemonic: "616c6c20686f7572206d616b65206669727374206c656164657220657874656e6420686f6c6520616c69656e20626568696e6420677561726420676f7370656c206c6176612070617468206f75747075742063656e737573206d757365756d206a756e696f72206d6173732072656f70656e2066616d6f75732073696e6720616476616e63652073616c74207265666f726d",
        passphrase: "5452455a4f52",
        seed: "26e975ec644423f4a4c4f4215ef09b4bd7ef924e85d1d17c4cf3f136c2863cf6df0a475045652c57eb5fb41513ca2a2d67722b77e954b4b3fc11f7590449191d",
    },
    TestVector {
        wordlist: wordlist::ENGLISH,
        entropy: "f30f8c1da665478f49b001d94c5fc452",
        mnemonic: "76657373656c206c616464657220616c746572206572726f72206665646572616c207369626c696e672063686174206162696c6974792073756e20676c6173732076616c76652070696374757265",
        passphrase: "5452455a4f52",
        seed: "2aaa9242daafcee6aa9d7269f17d4efe271e1b9a529178d7dc139cd18747090bf9d60295d0ce74309a78852a9caadf0af48aae1c6253839624076224374bc63f",
    },
    TestVector {
        wordlist: wordlist::ENGLISH,
        entropy: "c10ec20dc3cd9f652c7fac2f1230f7a3c828389a14392f05",
        mnemonic: "73636973736f727320696e76697465206c6f636b206d61706c652073757072656d652072617720726170696420766f696420636f6e6772657373206d7573636c65206469676974616c20656c6567616e74206c6974746c6520627269736b2068616972206d616e676f20636f6e677265737320636c756d70",
        passphrase: "5452455a4f52",
        seed: "7b4a10be9d98e6cba265566db7f136718e1398c71cb581e1b2f464cac1ceedf4f3e274dc270003c670ad8d02c4558b2f8e39edea2775c9e232c7cb798b069e88",
    },
    TestVector {
        wordlist: wordlist::ENGLISH,
        entropy: "f585c11aec520db57dd353c69554b21a89b20fb0650966fa0a9d6f74fd989d8f",
        mnemonic: "766f696420636f6d65206566666f7274207375666665722063616d70207375727665792077617272696f722068656176792073686f6f74207072696d61727920636c75746368206372757368206f70656e20616d617a696e672073637265656e20706174726f6c2067726f757020737061636520706f696e742074656e20657869737420736c75736820696e766f6c766520756e666f6c64",
        passphrase: "5452455a4f52",
        seed: "01f5bced59dec48e362f2c45b5de68b9fd6c92c6634f44d6d40aab69056506f0e35524a518034ddc1192e1dacd32c1ed3eaa3c3b131c88ed8e7e54c49a5d0998",
    },
]