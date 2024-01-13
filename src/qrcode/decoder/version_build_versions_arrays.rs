use super::{ECBlocks, Version, ECB};

impl Version {
    pub fn build_micro_versions() -> Vec<Version> {
        vec![
            Version::new_micro(1, vec![ECBlocks::new(2, vec![ECB::new(1, 3)])]),
            Version::new_micro(
                2,
                vec![
                    ECBlocks::new(5, vec![ECB::new(1, 5)]),
                    ECBlocks::new(6, vec![ECB::new(1, 4)]),
                ],
            ),
            Version::new_micro(
                3,
                vec![
                    ECBlocks::new(6, vec![ECB::new(1, 11)]),
                    ECBlocks::new(8, vec![ECB::new(1, 9)]),
                ],
            ),
            Version::new_micro(
                4,
                vec![
                    ECBlocks::new(8, vec![ECB::new(1, 16)]),
                    ECBlocks::new(10, vec![ECB::new(1, 14)]),
                    ECBlocks::new(14, vec![ECB::new(1, 10)]),
                ],
            ),
        ]
        // static const Version allVersions[] = {
        // 	{1, {2, 1, 3, 0, 0}},
        // 	{2, {5, 1, 5, 0, 0, 6, 1, 4, 0, 0}},
        // 	{3, {6, 1, 11, 0, 0, 8, 1, 9, 0, 0}},
        // 	{4, {8, 1, 16, 0, 0, 10, 1, 14, 0, 0, 14, 1, 10, 0, 0}}};
    }

    /**
     * See ISO 18004:2006 6.5.1 Table 9
     */
    pub fn buildVersions() -> Vec<Version> {
        Vec::from([
            Version::new(
                1,
                Vec::from([]),
                [
                    ECBlocks::new(7, Vec::from([ECB::new(1, 19)])),
                    ECBlocks::new(10, Vec::from([ECB::new(1, 16)])),
                    ECBlocks::new(13, Vec::from([ECB::new(1, 13)])),
                    ECBlocks::new(17, Vec::from([ECB::new(1, 9)])),
                ],
            ),
            Version::new(
                2,
                Vec::from([6, 18]),
                [
                    ECBlocks::new(10, Vec::from([ECB::new(1, 34)])),
                    ECBlocks::new(16, Vec::from([ECB::new(1, 28)])),
                    ECBlocks::new(22, Vec::from([ECB::new(1, 22)])),
                    ECBlocks::new(28, Vec::from([ECB::new(1, 16)])),
                ],
            ),
            Version::new(
                3,
                Vec::from([6, 22]),
                [
                    ECBlocks::new(15, Vec::from([ECB::new(1, 55)])),
                    ECBlocks::new(26, Vec::from([ECB::new(1, 44)])),
                    ECBlocks::new(18, Vec::from([ECB::new(2, 17)])),
                    ECBlocks::new(22, Vec::from([ECB::new(2, 13)])),
                ],
            ),
            Version::new(
                4,
                Vec::from([6, 26]),
                [
                    ECBlocks::new(20, Vec::from([ECB::new(1, 80)])),
                    ECBlocks::new(18, Vec::from([ECB::new(2, 32)])),
                    ECBlocks::new(26, Vec::from([ECB::new(2, 24)])),
                    ECBlocks::new(16, Vec::from([ECB::new(4, 9)])),
                ],
            ),
            Version::new(
                5,
                Vec::from([6, 30]),
                [
                    ECBlocks::new(26, Vec::from([ECB::new(1, 108)])),
                    ECBlocks::new(24, Vec::from([ECB::new(2, 43)])),
                    ECBlocks::new(18, Vec::from([ECB::new(2, 15), ECB::new(2, 16)])),
                    ECBlocks::new(22, Vec::from([ECB::new(2, 11), ECB::new(2, 12)])),
                ],
            ),
            Version::new(
                6,
                Vec::from([6, 34]),
                [
                    ECBlocks::new(18, Vec::from([ECB::new(2, 68)])),
                    ECBlocks::new(16, Vec::from([ECB::new(4, 27)])),
                    ECBlocks::new(24, Vec::from([ECB::new(4, 19)])),
                    ECBlocks::new(28, Vec::from([ECB::new(4, 15)])),
                ],
            ),
            Version::new(
                7,
                Vec::from([6, 22, 38]),
                [
                    ECBlocks::new(20, Vec::from([ECB::new(2, 78)])),
                    ECBlocks::new(18, Vec::from([ECB::new(4, 31)])),
                    ECBlocks::new(18, Vec::from([ECB::new(2, 14), ECB::new(4, 15)])),
                    ECBlocks::new(26, Vec::from([ECB::new(4, 13), ECB::new(1, 14)])),
                ],
            ),
            Version::new(
                8,
                Vec::from([6, 24, 42]),
                [
                    ECBlocks::new(24, Vec::from([ECB::new(2, 97)])),
                    ECBlocks::new(22, Vec::from([ECB::new(2, 38), ECB::new(2, 39)])),
                    ECBlocks::new(22, Vec::from([ECB::new(4, 18), ECB::new(2, 19)])),
                    ECBlocks::new(26, Vec::from([ECB::new(4, 14), ECB::new(2, 15)])),
                ],
            ),
            Version::new(
                9,
                Vec::from([6, 26, 46]),
                [
                    ECBlocks::new(30, Vec::from([ECB::new(2, 116)])),
                    ECBlocks::new(22, Vec::from([ECB::new(3, 36), ECB::new(2, 37)])),
                    ECBlocks::new(20, Vec::from([ECB::new(4, 16), ECB::new(4, 17)])),
                    ECBlocks::new(24, Vec::from([ECB::new(4, 12), ECB::new(4, 13)])),
                ],
            ),
            Version::new(
                10,
                Vec::from([6, 28, 50]),
                [
                    ECBlocks::new(18, Vec::from([ECB::new(2, 68), ECB::new(2, 69)])),
                    ECBlocks::new(26, Vec::from([ECB::new(4, 43), ECB::new(1, 44)])),
                    ECBlocks::new(24, Vec::from([ECB::new(6, 19), ECB::new(2, 20)])),
                    ECBlocks::new(28, Vec::from([ECB::new(6, 15), ECB::new(2, 16)])),
                ],
            ),
            Version::new(
                11,
                Vec::from([6, 30, 54]),
                [
                    ECBlocks::new(20, Vec::from([ECB::new(4, 81)])),
                    ECBlocks::new(30, Vec::from([ECB::new(1, 50), ECB::new(4, 51)])),
                    ECBlocks::new(28, Vec::from([ECB::new(4, 22), ECB::new(4, 23)])),
                    ECBlocks::new(24, Vec::from([ECB::new(3, 12), ECB::new(8, 13)])),
                ],
            ),
            Version::new(
                12,
                Vec::from([6, 32, 58]),
                [
                    ECBlocks::new(24, Vec::from([ECB::new(2, 92), ECB::new(2, 93)])),
                    ECBlocks::new(22, Vec::from([ECB::new(6, 36), ECB::new(2, 37)])),
                    ECBlocks::new(26, Vec::from([ECB::new(4, 20), ECB::new(6, 21)])),
                    ECBlocks::new(28, Vec::from([ECB::new(7, 14), ECB::new(4, 15)])),
                ],
            ),
            Version::new(
                13,
                Vec::from([6, 34, 62]),
                [
                    ECBlocks::new(26, Vec::from([ECB::new(4, 107)])),
                    ECBlocks::new(22, Vec::from([ECB::new(8, 37), ECB::new(1, 38)])),
                    ECBlocks::new(24, Vec::from([ECB::new(8, 20), ECB::new(4, 21)])),
                    ECBlocks::new(22, Vec::from([ECB::new(12, 11), ECB::new(4, 12)])),
                ],
            ),
            Version::new(
                14,
                Vec::from([6, 26, 46, 66]),
                [
                    ECBlocks::new(30, Vec::from([ECB::new(3, 115), ECB::new(1, 116)])),
                    ECBlocks::new(24, Vec::from([ECB::new(4, 40), ECB::new(5, 41)])),
                    ECBlocks::new(20, Vec::from([ECB::new(11, 16), ECB::new(5, 17)])),
                    ECBlocks::new(24, Vec::from([ECB::new(11, 12), ECB::new(5, 13)])),
                ],
            ),
            Version::new(
                15,
                Vec::from([6, 26, 48, 70]),
                [
                    ECBlocks::new(22, Vec::from([ECB::new(5, 87), ECB::new(1, 88)])),
                    ECBlocks::new(24, Vec::from([ECB::new(5, 41), ECB::new(5, 42)])),
                    ECBlocks::new(30, Vec::from([ECB::new(5, 24), ECB::new(7, 25)])),
                    ECBlocks::new(24, Vec::from([ECB::new(11, 12), ECB::new(7, 13)])),
                ],
            ),
            Version::new(
                16,
                Vec::from([6, 26, 50, 74]),
                [
                    ECBlocks::new(24, Vec::from([ECB::new(5, 98), ECB::new(1, 99)])),
                    ECBlocks::new(28, Vec::from([ECB::new(7, 45), ECB::new(3, 46)])),
                    ECBlocks::new(24, Vec::from([ECB::new(15, 19), ECB::new(2, 20)])),
                    ECBlocks::new(30, Vec::from([ECB::new(3, 15), ECB::new(13, 16)])),
                ],
            ),
            Version::new(
                17,
                Vec::from([6, 30, 54, 78]),
                [
                    ECBlocks::new(28, Vec::from([ECB::new(1, 107), ECB::new(5, 108)])),
                    ECBlocks::new(28, Vec::from([ECB::new(10, 46), ECB::new(1, 47)])),
                    ECBlocks::new(28, Vec::from([ECB::new(1, 22), ECB::new(15, 23)])),
                    ECBlocks::new(28, Vec::from([ECB::new(2, 14), ECB::new(17, 15)])),
                ],
            ),
            Version::new(
                18,
                Vec::from([6, 30, 56, 82]),
                [
                    ECBlocks::new(30, Vec::from([ECB::new(5, 120), ECB::new(1, 121)])),
                    ECBlocks::new(26, Vec::from([ECB::new(9, 43), ECB::new(4, 44)])),
                    ECBlocks::new(28, Vec::from([ECB::new(17, 22), ECB::new(1, 23)])),
                    ECBlocks::new(28, Vec::from([ECB::new(2, 14), ECB::new(19, 15)])),
                ],
            ),
            Version::new(
                19,
                Vec::from([6, 30, 58, 86]),
                [
                    ECBlocks::new(28, Vec::from([ECB::new(3, 113), ECB::new(4, 114)])),
                    ECBlocks::new(26, Vec::from([ECB::new(3, 44), ECB::new(11, 45)])),
                    ECBlocks::new(26, Vec::from([ECB::new(17, 21), ECB::new(4, 22)])),
                    ECBlocks::new(26, Vec::from([ECB::new(9, 13), ECB::new(16, 14)])),
                ],
            ),
            Version::new(
                20,
                Vec::from([6, 34, 62, 90]),
                [
                    ECBlocks::new(28, Vec::from([ECB::new(3, 107), ECB::new(5, 108)])),
                    ECBlocks::new(26, Vec::from([ECB::new(3, 41), ECB::new(13, 42)])),
                    ECBlocks::new(30, Vec::from([ECB::new(15, 24), ECB::new(5, 25)])),
                    ECBlocks::new(28, Vec::from([ECB::new(15, 15), ECB::new(10, 16)])),
                ],
            ),
            Version::new(
                21,
                Vec::from([6, 28, 50, 72, 94]),
                [
                    ECBlocks::new(28, Vec::from([ECB::new(4, 116), ECB::new(4, 117)])),
                    ECBlocks::new(26, Vec::from([ECB::new(17, 42)])),
                    ECBlocks::new(28, Vec::from([ECB::new(17, 22), ECB::new(6, 23)])),
                    ECBlocks::new(30, Vec::from([ECB::new(19, 16), ECB::new(6, 17)])),
                ],
            ),
            Version::new(
                22,
                Vec::from([6, 26, 50, 74, 98]),
                [
                    ECBlocks::new(28, Vec::from([ECB::new(2, 111), ECB::new(7, 112)])),
                    ECBlocks::new(28, Vec::from([ECB::new(17, 46)])),
                    ECBlocks::new(30, Vec::from([ECB::new(7, 24), ECB::new(16, 25)])),
                    ECBlocks::new(24, Vec::from([ECB::new(34, 13)])),
                ],
            ),
            Version::new(
                23,
                Vec::from([6, 30, 54, 78, 102]),
                [
                    ECBlocks::new(30, Vec::from([ECB::new(4, 121), ECB::new(5, 122)])),
                    ECBlocks::new(28, Vec::from([ECB::new(4, 47), ECB::new(14, 48)])),
                    ECBlocks::new(30, Vec::from([ECB::new(11, 24), ECB::new(14, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(16, 15), ECB::new(14, 16)])),
                ],
            ),
            Version::new(
                24,
                Vec::from([6, 28, 54, 80, 106]),
                [
                    ECBlocks::new(30, Vec::from([ECB::new(6, 117), ECB::new(4, 118)])),
                    ECBlocks::new(28, Vec::from([ECB::new(6, 45), ECB::new(14, 46)])),
                    ECBlocks::new(30, Vec::from([ECB::new(11, 24), ECB::new(16, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(30, 16), ECB::new(2, 17)])),
                ],
            ),
            Version::new(
                25,
                Vec::from([6, 32, 58, 84, 110]),
                [
                    ECBlocks::new(26, Vec::from([ECB::new(8, 106), ECB::new(4, 107)])),
                    ECBlocks::new(28, Vec::from([ECB::new(8, 47), ECB::new(13, 48)])),
                    ECBlocks::new(30, Vec::from([ECB::new(7, 24), ECB::new(22, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(22, 15), ECB::new(13, 16)])),
                ],
            ),
            Version::new(
                26,
                Vec::from([6, 30, 58, 86, 114]),
                [
                    ECBlocks::new(28, Vec::from([ECB::new(10, 114), ECB::new(2, 115)])),
                    ECBlocks::new(28, Vec::from([ECB::new(19, 46), ECB::new(4, 47)])),
                    ECBlocks::new(28, Vec::from([ECB::new(28, 22), ECB::new(6, 23)])),
                    ECBlocks::new(30, Vec::from([ECB::new(33, 16), ECB::new(4, 17)])),
                ],
            ),
            Version::new(
                27,
                Vec::from([6, 34, 62, 90, 118]),
                [
                    ECBlocks::new(30, Vec::from([ECB::new(8, 122), ECB::new(4, 123)])),
                    ECBlocks::new(28, Vec::from([ECB::new(22, 45), ECB::new(3, 46)])),
                    ECBlocks::new(30, Vec::from([ECB::new(8, 23), ECB::new(26, 24)])),
                    ECBlocks::new(30, Vec::from([ECB::new(12, 15), ECB::new(28, 16)])),
                ],
            ),
            Version::new(
                28,
                Vec::from([6, 26, 50, 74, 98, 122]),
                [
                    ECBlocks::new(30, Vec::from([ECB::new(3, 117), ECB::new(10, 118)])),
                    ECBlocks::new(28, Vec::from([ECB::new(3, 45), ECB::new(23, 46)])),
                    ECBlocks::new(30, Vec::from([ECB::new(4, 24), ECB::new(31, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(11, 15), ECB::new(31, 16)])),
                ],
            ),
            Version::new(
                29,
                Vec::from([6, 30, 54, 78, 102, 126]),
                [
                    ECBlocks::new(30, Vec::from([ECB::new(7, 116), ECB::new(7, 117)])),
                    ECBlocks::new(28, Vec::from([ECB::new(21, 45), ECB::new(7, 46)])),
                    ECBlocks::new(30, Vec::from([ECB::new(1, 23), ECB::new(37, 24)])),
                    ECBlocks::new(30, Vec::from([ECB::new(19, 15), ECB::new(26, 16)])),
                ],
            ),
            Version::new(
                30,
                Vec::from([6, 26, 52, 78, 104, 130]),
                [
                    ECBlocks::new(30, Vec::from([ECB::new(5, 115), ECB::new(10, 116)])),
                    ECBlocks::new(28, Vec::from([ECB::new(19, 47), ECB::new(10, 48)])),
                    ECBlocks::new(30, Vec::from([ECB::new(15, 24), ECB::new(25, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(23, 15), ECB::new(25, 16)])),
                ],
            ),
            Version::new(
                31,
                Vec::from([6, 30, 56, 82, 108, 134]),
                [
                    ECBlocks::new(30, Vec::from([ECB::new(13, 115), ECB::new(3, 116)])),
                    ECBlocks::new(28, Vec::from([ECB::new(2, 46), ECB::new(29, 47)])),
                    ECBlocks::new(30, Vec::from([ECB::new(42, 24), ECB::new(1, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(23, 15), ECB::new(28, 16)])),
                ],
            ),
            Version::new(
                32,
                Vec::from([6, 34, 60, 86, 112, 138]),
                [
                    ECBlocks::new(30, Vec::from([ECB::new(17, 115)])),
                    ECBlocks::new(28, Vec::from([ECB::new(10, 46), ECB::new(23, 47)])),
                    ECBlocks::new(30, Vec::from([ECB::new(10, 24), ECB::new(35, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(19, 15), ECB::new(35, 16)])),
                ],
            ),
            Version::new(
                33,
                Vec::from([6, 30, 58, 86, 114, 142]),
                [
                    ECBlocks::new(30, Vec::from([ECB::new(17, 115), ECB::new(1, 116)])),
                    ECBlocks::new(28, Vec::from([ECB::new(14, 46), ECB::new(21, 47)])),
                    ECBlocks::new(30, Vec::from([ECB::new(29, 24), ECB::new(19, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(11, 15), ECB::new(46, 16)])),
                ],
            ),
            Version::new(
                34,
                Vec::from([6, 34, 62, 90, 118, 146]),
                [
                    ECBlocks::new(30, Vec::from([ECB::new(13, 115), ECB::new(6, 116)])),
                    ECBlocks::new(28, Vec::from([ECB::new(14, 46), ECB::new(23, 47)])),
                    ECBlocks::new(30, Vec::from([ECB::new(44, 24), ECB::new(7, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(59, 16), ECB::new(1, 17)])),
                ],
            ),
            Version::new(
                35,
                Vec::from([6, 30, 54, 78, 102, 126, 150]),
                [
                    ECBlocks::new(30, Vec::from([ECB::new(12, 121), ECB::new(7, 122)])),
                    ECBlocks::new(28, Vec::from([ECB::new(12, 47), ECB::new(26, 48)])),
                    ECBlocks::new(30, Vec::from([ECB::new(39, 24), ECB::new(14, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(22, 15), ECB::new(41, 16)])),
                ],
            ),
            Version::new(
                36,
                Vec::from([6, 24, 50, 76, 102, 128, 154]),
                [
                    ECBlocks::new(30, Vec::from([ECB::new(6, 121), ECB::new(14, 122)])),
                    ECBlocks::new(28, Vec::from([ECB::new(6, 47), ECB::new(34, 48)])),
                    ECBlocks::new(30, Vec::from([ECB::new(46, 24), ECB::new(10, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(2, 15), ECB::new(64, 16)])),
                ],
            ),
            Version::new(
                37,
                Vec::from([6, 28, 54, 80, 106, 132, 158]),
                [
                    ECBlocks::new(30, Vec::from([ECB::new(17, 122), ECB::new(4, 123)])),
                    ECBlocks::new(28, Vec::from([ECB::new(29, 46), ECB::new(14, 47)])),
                    ECBlocks::new(30, Vec::from([ECB::new(49, 24), ECB::new(10, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(24, 15), ECB::new(46, 16)])),
                ],
            ),
            Version::new(
                38,
                Vec::from([6, 32, 58, 84, 110, 136, 162]),
                [
                    ECBlocks::new(30, Vec::from([ECB::new(4, 122), ECB::new(18, 123)])),
                    ECBlocks::new(28, Vec::from([ECB::new(13, 46), ECB::new(32, 47)])),
                    ECBlocks::new(30, Vec::from([ECB::new(48, 24), ECB::new(14, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(42, 15), ECB::new(32, 16)])),
                ],
            ),
            Version::new(
                39,
                Vec::from([6, 26, 54, 82, 110, 138, 166]),
                [
                    ECBlocks::new(30, Vec::from([ECB::new(20, 117), ECB::new(4, 118)])),
                    ECBlocks::new(28, Vec::from([ECB::new(40, 47), ECB::new(7, 48)])),
                    ECBlocks::new(30, Vec::from([ECB::new(43, 24), ECB::new(22, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(10, 15), ECB::new(67, 16)])),
                ],
            ),
            Version::new(
                40,
                Vec::from([6, 30, 58, 86, 114, 142, 170]),
                [
                    ECBlocks::new(30, Vec::from([ECB::new(19, 118), ECB::new(6, 119)])),
                    ECBlocks::new(28, Vec::from([ECB::new(18, 47), ECB::new(31, 48)])),
                    ECBlocks::new(30, Vec::from([ECB::new(34, 24), ECB::new(34, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(20, 15), ECB::new(61, 16)])),
                ],
            ),
        ]) /*
               new Version(4, new int[]{6, 26},
                   new ECBlocks(20, new ECB::new(1, 80)),
                   new ECBlocks(18, new ECB::new(2, 32)),
                   new ECBlocks(26, new ECB::new(2, 24)),
                   new ECBlocks(16, new ECB::new(4, 9))),
               new Version(5, new int[]{6, 30},
                   new ECBlocks(26, new ECB::new(1, 108)),
                   new ECBlocks(24, new ECB::new(2, 43)),
                   new ECBlocks(18, new ECB::new(2, 15),
                       new ECB::new(2, 16)),
                   new ECBlocks(22, new ECB::new(2, 11),
                       new ECB::new(2, 12))),
               new Version(6, new int[]{6, 34},
                   new ECBlocks(18, new ECB::new(2, 68)),
                   new ECBlocks(16, new ECB::new(4, 27)),
                   new ECBlocks(24, new ECB::new(4, 19)),
                   new ECBlocks(28, new ECB::new(4, 15))),
               new Version(7, new int[]{6, 22, 38},
                   new ECBlocks(20, new ECB::new(2, 78)),
                   new ECBlocks(18, new ECB::new(4, 31)),
                   new ECBlocks(18, new ECB::new(2, 14),
                       new ECB::new(4, 15)),
                   new ECBlocks(26, new ECB::new(4, 13),
                       new ECB::new(1, 14))),
               new Version(8, new int[]{6, 24, 42},
                   new ECBlocks(24, new ECB::new(2, 97)),
                   new ECBlocks(22, new ECB::new(2, 38),
                       new ECB::new(2, 39)),
                   new ECBlocks(22, new ECB::new(4, 18),
                       new ECB::new(2, 19)),
                   new ECBlocks(26, new ECB::new(4, 14),
                       new ECB::new(2, 15))),
               new Version(9, new int[]{6, 26, 46},
                   new ECBlocks(30, new ECB::new(2, 116)),
                   new ECBlocks(22, new ECB::new(3, 36),
                       new ECB::new(2, 37)),
                   new ECBlocks(20, new ECB::new(4, 16),
                       new ECB::new(4, 17)),
                   new ECBlocks(24, new ECB::new(4, 12),
                       new ECB::new(4, 13))),
               new Version(10, new int[]{6, 28, 50},
                   new ECBlocks(18, new ECB::new(2, 68),
                       new ECB::new(2, 69)),
                   new ECBlocks(26, new ECB::new(4, 43),
                       new ECB::new(1, 44)),
                   new ECBlocks(24, new ECB::new(6, 19),
                       new ECB::new(2, 20)),
                   new ECBlocks(28, new ECB::new(6, 15),
                       new ECB::new(2, 16))),
               new Version(11, new int[]{6, 30, 54},
                   new ECBlocks(20, new ECB::new(4, 81)),
                   new ECBlocks(30, new ECB::new(1, 50),
                       new ECB::new(4, 51)),
                   new ECBlocks(28, new ECB::new(4, 22),
                       new ECB::new(4, 23)),
                   new ECBlocks(24, new ECB::new(3, 12),
                       new ECB::new(8, 13))),
               new Version(12, new int[]{6, 32, 58},
                   new ECBlocks(24, new ECB::new(2, 92),
                       new ECB::new(2, 93)),
                   new ECBlocks(22, new ECB::new(6, 36),
                       new ECB::new(2, 37)),
                   new ECBlocks(26, new ECB::new(4, 20),
                       new ECB::new(6, 21)),
                   new ECBlocks(28, new ECB::new(7, 14),
                       new ECB::new(4, 15))),
               new Version(13, new int[]{6, 34, 62},
                   new ECBlocks(26, new ECB::new(4, 107)),
                   new ECBlocks(22, new ECB::new(8, 37),
                       new ECB::new(1, 38)),
                   new ECBlocks(24, new ECB::new(8, 20),
                       new ECB::new(4, 21)),
                   new ECBlocks(22, new ECB::new(12, 11),
                       new ECB::new(4, 12))),
               new Version(14, new int[]{6, 26, 46, 66},
                   new ECBlocks(30, new ECB::new(3, 115),
                       new ECB::new(1, 116)),
                   new ECBlocks(24, new ECB::new(4, 40),
                       new ECB::new(5, 41)),
                   new ECBlocks(20, new ECB::new(11, 16),
                       new ECB::new(5, 17)),
                   new ECBlocks(24, new ECB::new(11, 12),
                       new ECB::new(5, 13))),
               new Version(15, new int[]{6, 26, 48, 70},
                   new ECBlocks(22, new ECB::new(5, 87),
                       new ECB::new(1, 88)),
                   new ECBlocks(24, new ECB::new(5, 41),
                       new ECB::new(5, 42)),
                   new ECBlocks(30, new ECB::new(5, 24),
                       new ECB::new(7, 25)),
                   new ECBlocks(24, new ECB::new(11, 12),
                       new ECB::new(7, 13))),
               new Version(16, new int[]{6, 26, 50, 74},
                   new ECBlocks(24, new ECB::new(5, 98),
                       new ECB::new(1, 99)),
                   new ECBlocks(28, new ECB::new(7, 45),
                       new ECB::new(3, 46)),
                   new ECBlocks(24, new ECB::new(15, 19),
                       new ECB::new(2, 20)),
                   new ECBlocks(30, new ECB::new(3, 15),
                       new ECB::new(13, 16))),
               new Version(17, new int[]{6, 30, 54, 78},
                   new ECBlocks(28, new ECB::new(1, 107),
                       new ECB::new(5, 108)),
                   new ECBlocks(28, new ECB::new(10, 46),
                       new ECB::new(1, 47)),
                   new ECBlocks(28, new ECB::new(1, 22),
                       new ECB::new(15, 23)),
                   new ECBlocks(28, new ECB::new(2, 14),
                       new ECB::new(17, 15))),
               new Version(18, new int[]{6, 30, 56, 82},
                   new ECBlocks(30, new ECB::new(5, 120),
                       new ECB::new(1, 121)),
                   new ECBlocks(26, new ECB::new(9, 43),
                       new ECB::new(4, 44)),
                   new ECBlocks(28, new ECB::new(17, 22),
                       new ECB::new(1, 23)),
                   new ECBlocks(28, new ECB::new(2, 14),
                       new ECB::new(19, 15))),
               new Version(19, new int[]{6, 30, 58, 86},
                   new ECBlocks(28, new ECB::new(3, 113),
                       new ECB::new(4, 114)),
                   new ECBlocks(26, new ECB::new(3, 44),
                       new ECB::new(11, 45)),
                   new ECBlocks(26, new ECB::new(17, 21),
                       new ECB::new(4, 22)),
                   new ECBlocks(26, new ECB::new(9, 13),
                       new ECB::new(16, 14))),
               new Version(20, new int[]{6, 34, 62, 90},
                   new ECBlocks(28, new ECB::new(3, 107),
                       new ECB::new(5, 108)),
                   new ECBlocks(26, new ECB::new(3, 41),
                       new ECB::new(13, 42)),
                   new ECBlocks(30, new ECB::new(15, 24),
                       new ECB::new(5, 25)),
                   new ECBlocks(28, new ECB::new(15, 15),
                       new ECB::new(10, 16))),
               new Version(21, new int[]{6, 28, 50, 72, 94},
                   new ECBlocks(28, new ECB::new(4, 116),
                       new ECB::new(4, 117)),
                   new ECBlocks(26, new ECB::new(17, 42)),
                   new ECBlocks(28, new ECB::new(17, 22),
                       new ECB::new(6, 23)),
                   new ECBlocks(30, new ECB::new(19, 16),
                       new ECB::new(6, 17))),
               new Version(22, new int[]{6, 26, 50, 74, 98},
                   new ECBlocks(28, new ECB::new(2, 111),
                       new ECB::new(7, 112)),
                   new ECBlocks(28, new ECB::new(17, 46)),
                   new ECBlocks(30, new ECB::new(7, 24),
                       new ECB::new(16, 25)),
                   new ECBlocks(24, new ECB::new(34, 13))),
               new Version(23, new int[]{6, 30, 54, 78, 102},
                   new ECBlocks(30, new ECB::new(4, 121),
                       new ECB::new(5, 122)),
                   new ECBlocks(28, new ECB::new(4, 47),
                       new ECB::new(14, 48)),
                   new ECBlocks(30, new ECB::new(11, 24),
                       new ECB::new(14, 25)),
                   new ECBlocks(30, new ECB::new(16, 15),
                       new ECB::new(14, 16))),
               new Version(24, new int[]{6, 28, 54, 80, 106},
                   new ECBlocks(30, new ECB::new(6, 117),
                       new ECB::new(4, 118)),
                   new ECBlocks(28, new ECB::new(6, 45),
                       new ECB::new(14, 46)),
                   new ECBlocks(30, new ECB::new(11, 24),
                       new ECB::new(16, 25)),
                   new ECBlocks(30, new ECB::new(30, 16),
                       new ECB::new(2, 17))),
               new Version(25, new int[]{6, 32, 58, 84, 110},
                   new ECBlocks(26, new ECB::new(8, 106),
                       new ECB::new(4, 107)),
                   new ECBlocks(28, new ECB::new(8, 47),
                       new ECB::new(13, 48)),
                   new ECBlocks(30, new ECB::new(7, 24),
                       new ECB::new(22, 25)),
                   new ECBlocks(30, new ECB::new(22, 15),
                       new ECB::new(13, 16))),
               new Version(26, new int[]{6, 30, 58, 86, 114},
                   new ECBlocks(28, new ECB::new(10, 114),
                       new ECB::new(2, 115)),
                   new ECBlocks(28, new ECB::new(19, 46),
                       new ECB::new(4, 47)),
                   new ECBlocks(28, new ECB::new(28, 22),
                       new ECB::new(6, 23)),
                   new ECBlocks(30, new ECB::new(33, 16),
                       new ECB::new(4, 17))),
               new Version(27, new int[]{6, 34, 62, 90, 118},
                   new ECBlocks(30, new ECB::new(8, 122),
                       new ECB::new(4, 123)),
                   new ECBlocks(28, new ECB::new(22, 45),
                       new ECB::new(3, 46)),
                   new ECBlocks(30, new ECB::new(8, 23),
                       new ECB::new(26, 24)),
                   new ECBlocks(30, new ECB::new(12, 15),
                       new ECB::new(28, 16))),
               new Version(28, new int[]{6, 26, 50, 74, 98, 122},
                   new ECBlocks(30, new ECB::new(3, 117),
                       new ECB::new(10, 118)),
                   new ECBlocks(28, new ECB::new(3, 45),
                       new ECB::new(23, 46)),
                   new ECBlocks(30, new ECB::new(4, 24),
                       new ECB::new(31, 25)),
                   new ECBlocks(30, new ECB::new(11, 15),
                       new ECB::new(31, 16))),
               new Version(29, new int[]{6, 30, 54, 78, 102, 126},
                   new ECBlocks(30, new ECB::new(7, 116),
                       new ECB::new(7, 117)),
                   new ECBlocks(28, new ECB::new(21, 45),
                       new ECB::new(7, 46)),
                   new ECBlocks(30, new ECB::new(1, 23),
                       new ECB::new(37, 24)),
                   new ECBlocks(30, new ECB::new(19, 15),
                       new ECB::new(26, 16))),
               new Version(30, new int[]{6, 26, 52, 78, 104, 130},
                   new ECBlocks(30, new ECB::new(5, 115),
                       new ECB::new(10, 116)),
                   new ECBlocks(28, new ECB::new(19, 47),
                       new ECB::new(10, 48)),
                   new ECBlocks(30, new ECB::new(15, 24),
                       new ECB::new(25, 25)),
                   new ECBlocks(30, new ECB::new(23, 15),
                       new ECB::new(25, 16))),
               new Version(31, new int[]{6, 30, 56, 82, 108, 134},
                   new ECBlocks(30, new ECB::new(13, 115),
                       new ECB::new(3, 116)),
                   new ECBlocks(28, new ECB::new(2, 46),
                       new ECB::new(29, 47)),
                   new ECBlocks(30, new ECB::new(42, 24),
                       new ECB::new(1, 25)),
                   new ECBlocks(30, new ECB::new(23, 15),
                       new ECB::new(28, 16))),
               new Version(32, new int[]{6, 34, 60, 86, 112, 138},
                   new ECBlocks(30, new ECB::new(17, 115)),
                   new ECBlocks(28, new ECB::new(10, 46),
                       new ECB::new(23, 47)),
                   new ECBlocks(30, new ECB::new(10, 24),
                       new ECB::new(35, 25)),
                   new ECBlocks(30, new ECB::new(19, 15),
                       new ECB::new(35, 16))),
               new Version(33, new int[]{6, 30, 58, 86, 114, 142},
                   new ECBlocks(30, new ECB::new(17, 115),
                       new ECB::new(1, 116)),
                   new ECBlocks(28, new ECB::new(14, 46),
                       new ECB::new(21, 47)),
                   new ECBlocks(30, new ECB::new(29, 24),
                       new ECB::new(19, 25)),
                   new ECBlocks(30, new ECB::new(11, 15),
                       new ECB::new(46, 16))),
               new Version(34, new int[]{6, 34, 62, 90, 118, 146},
                   new ECBlocks(30, new ECB::new(13, 115),
                       new ECB::new(6, 116)),
                   new ECBlocks(28, new ECB::new(14, 46),
                       new ECB::new(23, 47)),
                   new ECBlocks(30, new ECB::new(44, 24),
                       new ECB::new(7, 25)),
                   new ECBlocks(30, new ECB::new(59, 16),
                       new ECB::new(1, 17))),
               new Version(35, new int[]{6, 30, 54, 78, 102, 126, 150},
                   new ECBlocks(30, new ECB::new(12, 121),
                       new ECB::new(7, 122)),
                   new ECBlocks(28, new ECB::new(12, 47),
                       new ECB::new(26, 48)),
                   new ECBlocks(30, new ECB::new(39, 24),
                       new ECB::new(14, 25)),
                   new ECBlocks(30, new ECB::new(22, 15),
                       new ECB::new(41, 16))),
               new Version(36, new int[]{6, 24, 50, 76, 102, 128, 154},
                   new ECBlocks(30, new ECB::new(6, 121),
                       new ECB::new(14, 122)),
                   new ECBlocks(28, new ECB::new(6, 47),
                       new ECB::new(34, 48)),
                   new ECBlocks(30, new ECB::new(46, 24),
                       new ECB::new(10, 25)),
                   new ECBlocks(30, new ECB::new(2, 15),
                       new ECB::new(64, 16))),
               new Version(37, new int[]{6, 28, 54, 80, 106, 132, 158},
                   new ECBlocks(30, new ECB::new(17, 122),
                       new ECB::new(4, 123)),
                   new ECBlocks(28, new ECB::new(29, 46),
                       new ECB::new(14, 47)),
                   new ECBlocks(30, new ECB::new(49, 24),
                       new ECB::new(10, 25)),
                   new ECBlocks(30, new ECB::new(24, 15),
                       new ECB::new(46, 16))),
               new Version(38, new int[]{6, 32, 58, 84, 110, 136, 162},
                   new ECBlocks(30, new ECB::new(4, 122),
                       new ECB::new(18, 123)),
                   new ECBlocks(28, new ECB::new(13, 46),
                       new ECB::new(32, 47)),
                   new ECBlocks(30, new ECB::new(48, 24),
                       new ECB::new(14, 25)),
                   new ECBlocks(30, new ECB::new(42, 15),
                       new ECB::new(32, 16))),
               new Version(39, new int[]{6, 26, 54, 82, 110, 138, 166},
                   new ECBlocks(30, new ECB::new(20, 117),
                       new ECB::new(4, 118)),
                   new ECBlocks(28, new ECB::new(40, 47),
                       new ECB::new(7, 48)),
                   new ECBlocks(30, new ECB::new(43, 24),
                       new ECB::new(22, 25)),
                   new ECBlocks(30, new ECB::new(10, 15),
                       new ECB::new(67, 16))),
               new Version(40, new int[]{6, 30, 58, 86, 114, 142, 170},
                   new ECBlocks(30, new ECB::new(19, 118),
                       new ECB::new(6, 119)),
                   new ECBlocks(28, new ECB::new(18, 47),
                       new ECB::new(31, 48)),
                   new ECBlocks(30, new ECB::new(34, 24),
                       new ECB::new(34, 25)),
                   new ECBlocks(30, new ECB::new(20, 15),
                       new ECB::new(61, 16)))
           ]*/
    }

    /*
     * 		{1, {
            7 , 1, 19, 0, 0,
            10, 1, 16, 0, 0,
            13, 1, 13, 0, 0,
            17, 1, 9 , 0, 0
        }},
        {2, {
            10, 1, 36, 0, 0,
            16, 1, 30, 0, 0,
            22, 1, 24, 0, 0,
            30, 1, 16, 0, 0,
         }},
        {3, {
            15, 1, 57, 0, 0,
            28, 1, 44, 0, 0,
            36, 1, 36, 0, 0,
            48, 1, 24, 0, 0,
         }},
        {4, {
            20, 1, 80, 0, 0,
            40, 1, 60, 0, 0,
            50, 1, 50, 0, 0,
            66, 1, 34, 0, 0,
         }},
        {5, {
            26, 1, 108, 0, 0,
            52, 1, 82 , 0, 0,
            66, 1, 68 , 0, 0,
            88, 2, 46 , 0, 0,
            }},
        {6, {
            34 , 1, 136, 0, 0,
            63 , 2, 106, 0, 0,
            84 , 2, 86 , 0, 0,
            112, 2, 58 , 0, 0,
            }},
        {7, {
            42 , 1, 170, 0, 0,
            80 , 2, 132, 0, 0,
            104, 2, 108, 0, 0,
            138, 3, 72 , 0, 0,
            }},
        {8, {
            48 , 2, 208, 0, 0,
            96 , 2, 160, 0, 0,
            128, 2, 128, 0, 0,
            168, 3, 87 , 0, 0,
            }},
        {9, {
            60 , 2, 246, 0, 0,
            120, 2, 186, 0, 0,
            150, 3, 156, 0, 0,
            204, 3, 102, 0, 0,
            }},
        {10, {
            68 , 2, 290, 0, 0,
            136, 2, 222, 0, 0,
            174, 3, 183, 0, 0,
            232, 4, 124, 0, 0,
            }},
        {11, {
            80 , 2, 336, 0, 0,
            160, 4, 256, 0, 0,
            208, 4, 208, 0, 0,
            270, 5, 145, 0, 0,
            }},
        {12, {
            92 , 2, 384, 0, 0,
            184, 4, 292, 0, 0,
            232, 4, 244, 0, 0,
            310, 5, 165, 0, 0,
            }},
        {13, {
            108, 3, 432, 0, 0,
            208, 4, 332, 0, 0,
            264, 4, 276, 0, 0,
            348, 6, 192, 0, 0,
            }},
        {14, {
            120, 3, 489, 0, 0,
            240, 4, 368, 0, 0,
            300, 5, 310, 0, 0,
            396, 6, 210, 0, 0,
            }},
    };
     */
    pub fn build_model1_versions() -> Vec<Version> {
        Vec::from([
            Version::new_model1(
                1,
                vec![
                    ECBlocks::new(7, vec![ECB::new(1, 19)]),
                    ECBlocks::new(10, vec![ECB::new(1, 16)]),
                    ECBlocks::new(13, vec![ECB::new(1, 13)]),
                    ECBlocks::new(17, vec![ECB::new(1, 9)]),
                ],
            ),
            Version::new_model1(
                2,
                vec![
                    ECBlocks::new(10, vec![ECB::new(1, 36)]),
                    ECBlocks::new(16, vec![ECB::new(1, 30)]),
                    ECBlocks::new(22, vec![ECB::new(1, 24)]),
                    ECBlocks::new(30, vec![ECB::new(1, 16)]),
                ],
            ),
            Version::new_model1(
                3,
                vec![
                    ECBlocks::new(15, vec![ECB::new(1, 57)]),
                    ECBlocks::new(28, vec![ECB::new(1, 44)]),
                    ECBlocks::new(36, vec![ECB::new(1, 36)]),
                    ECBlocks::new(48, vec![ECB::new(1, 24)]),
                ],
            ),
            Version::new_model1(
                4,
                vec![
                    ECBlocks::new(20, vec![ECB::new(1, 80)]),
                    ECBlocks::new(40, vec![ECB::new(1, 60)]),
                    ECBlocks::new(50, vec![ECB::new(1, 50)]),
                    ECBlocks::new(66, vec![ECB::new(1, 34)]),
                ],
            ),
            Version::new_model1(
                5,
                vec![
                    ECBlocks::new(26, vec![ECB::new(1, 108)]),
                    ECBlocks::new(52, vec![ECB::new(1, 82)]),
                    ECBlocks::new(66, vec![ECB::new(1, 68)]),
                    ECBlocks::new(88, vec![ECB::new(2, 46)]),
                ],
            ),
            Version::new_model1(
                6,
                vec![
                    ECBlocks::new(34, vec![ECB::new(1, 136)]),
                    ECBlocks::new(63, vec![ECB::new(2, 106)]),
                    ECBlocks::new(84, vec![ECB::new(2, 86)]),
                    ECBlocks::new(112, vec![ECB::new(2, 58)]),
                ],
            ),
            Version::new_model1(
                7,
                vec![
                    ECBlocks::new(42, vec![ECB::new(1, 170)]),
                    ECBlocks::new(80, vec![ECB::new(2, 132)]),
                    ECBlocks::new(104, vec![ECB::new(2, 108)]),
                    ECBlocks::new(138, vec![ECB::new(3, 72)]),
                ],
            ),
            Version::new_model1(
                8,
                vec![
                    ECBlocks::new(48, vec![ECB::new(2, 208)]),
                    ECBlocks::new(96, vec![ECB::new(2, 160)]),
                    ECBlocks::new(128, vec![ECB::new(2, 128)]),
                    ECBlocks::new(168, vec![ECB::new(3, 87)]),
                ],
            ),
            Version::new_model1(
                9,
                vec![
                    ECBlocks::new(60, vec![ECB::new(2, 246)]),
                    ECBlocks::new(120, vec![ECB::new(2, 186)]),
                    ECBlocks::new(150, vec![ECB::new(3, 156)]),
                    ECBlocks::new(204, vec![ECB::new(3, 102)]),
                ],
            ),
            Version::new_model1(
                10,
                vec![
                    ECBlocks::new(68, vec![ECB::new(2, 290)]),
                    ECBlocks::new(136, vec![ECB::new(2, 222)]),
                    ECBlocks::new(174, vec![ECB::new(3, 183)]),
                    ECBlocks::new(232, vec![ECB::new(4, 124)]),
                ],
            ),
            Version::new_model1(
                11,
                vec![
                    ECBlocks::new(80, vec![ECB::new(2, 336)]),
                    ECBlocks::new(160, vec![ECB::new(4, 256)]),
                    ECBlocks::new(208, vec![ECB::new(4, 208)]),
                    ECBlocks::new(270, vec![ECB::new(5, 145)]),
                ],
            ),
            Version::new_model1(
                12,
                vec![
                    ECBlocks::new(92, vec![ECB::new(2, 384)]),
                    ECBlocks::new(184, vec![ECB::new(4, 292)]),
                    ECBlocks::new(232, vec![ECB::new(4, 244)]),
                    ECBlocks::new(310, vec![ECB::new(5, 165)]),
                ],
            ),
            Version::new_model1(
                13,
                vec![
                    ECBlocks::new(108, vec![ECB::new(3, 432)]),
                    ECBlocks::new(208, vec![ECB::new(4, 332)]),
                    ECBlocks::new(264, vec![ECB::new(4, 276)]),
                    ECBlocks::new(348, vec![ECB::new(6, 192)]),
                ],
            ),
            Version::new_model1(
                14,
                vec![
                    ECBlocks::new(120, vec![ECB::new(3, 489)]),
                    ECBlocks::new(240, vec![ECB::new(4, 368)]),
                    ECBlocks::new(300, vec![ECB::new(5, 310)]),
                    ECBlocks::new(396, vec![ECB::new(6, 210)]),
                ],
            ),
        ])
    }

    pub fn build_rmqr_versions() -> Vec<Version> {
        Vec::from([
            Version::new(
                1,
                Vec::from([21]),
                [
                    // R7x43
                    // 4 `ECBlocks`, one for each `ecLevel` - rMQR only uses M & H but using 2 dummies to keep `ecLevel` index same as QR Code
                    // Each begins with no. of error correction codewords divided by no. of error correction blocks, followed by 2 `ECBlock`s
                    // Each `ECBlock` begins with no. of error correction blocks followed by no. of data codewords per block
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])), // L (dummy) - also used to differentiate rMQR from Model2 in `Version::Version()`
                    ECBlocks::new(7, Vec::from([ECB::new(1, 6)])), // M
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])), // Q (dummy)
                    ECBlocks::new(10, Vec::from([ECB::new(1, 3)])), // H
                ],
            ),
            Version::new(
                2,
                Vec::from([19, 39]),
                [
                    // R7x59
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(9, Vec::from([ECB::new(1, 12)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(14, Vec::from([ECB::new(1, 7)])),
                ],
            ),
            Version::new(
                3,
                Vec::from([25, 51]),
                [
                    // R7x77
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(12, Vec::from([ECB::new(1, 20)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(22, Vec::from([ECB::new(1, 10)])),
                ],
            ),
            Version::new(
                4,
                Vec::from([23, 49, 75]),
                [
                    // R7x99
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(16, Vec::from([ECB::new(1, 28)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(30, Vec::from([ECB::new(1, 14)])),
                ],
            ),
            Version::new(
                5,
                Vec::from([27, 55, 83, 111]),
                [
                    // R7x139
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(24, Vec::from([ECB::new(1, 44)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(22, Vec::from([ECB::new(2, 12)])),
                ],
            ),
            Version::new(
                6,
                Vec::from([21]),
                [
                    // R9x43
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(9, Vec::from([ECB::new(1, 12)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(14, Vec::from([ECB::new(1, 7)])),
                ],
            ),
            Version::new(
                7,
                Vec::from([19, 39]),
                [
                    // R9x59
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(12, Vec::from([ECB::new(1, 21)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(22, Vec::from([ECB::new(1, 11)])),
                ],
            ),
            Version::new(
                8,
                Vec::from([25, 51]),
                [
                    // R9x77
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(18, Vec::from([ECB::new(1, 31)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(16, Vec::from([ECB::new(1, 8), ECB::new(1, 9)])),
                ],
            ),
            Version::new(
                9,
                Vec::from([23, 49, 75]),
                [
                    // R9x99
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(24, Vec::from([ECB::new(1, 42)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(22, Vec::from([ECB::new(2, 11)])),
                ],
            ),
            Version::new(
                10,
                Vec::from([27, 55, 83, 111]),
                [
                    // R9x139
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(18, Vec::from([ECB::new(1, 31), ECB::new(1, 32)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(22, Vec::from([ECB::new(3, 11)])),
                ],
            ),
            Version::new(
                11,
                Vec::from([]),
                [
                    // R11x27
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(8, Vec::from([ECB::new(1, 7)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(10, Vec::from([ECB::new(1, 5)])),
                ],
            ),
            Version::new(
                12,
                Vec::from([21]),
                [
                    // R11x43
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(12, Vec::from([ECB::new(1, 19)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(20, Vec::from([ECB::new(1, 11)])),
                ],
            ),
            Version::new(
                13,
                Vec::from([19, 39]),
                [
                    // R11x59
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(16, Vec::from([ECB::new(1, 31)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(16, Vec::from([ECB::new(1, 7), ECB::new(1, 8)])),
                ],
            ),
            Version::new(
                14,
                Vec::from([25, 51]),
                [
                    // R11x77
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(24, Vec::from([ECB::new(1, 43)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(22, Vec::from([ECB::new(1, 11), ECB::new(1, 12)])),
                ],
            ),
            Version::new(
                15,
                Vec::from([23, 49, 75]),
                [
                    // R11x99
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(16, Vec::from([ECB::new(1, 28), ECB::new(1, 29)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(30, Vec::from([ECB::new(1, 14), ECB::new(1, 15)])),
                ],
            ),
            Version::new(
                16,
                Vec::from([27, 55, 83, 111]),
                [
                    // R11x139
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(24, Vec::from([ECB::new(2, 42)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(30, Vec::from([ECB::new(3, 14)])),
                ],
            ),
            Version::new(
                17,
                Vec::from([]),
                [
                    // R13x27
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(9, Vec::from([ECB::new(1, 12)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(14, Vec::from([ECB::new(1, 7)])),
                ],
            ),
            Version::new(
                18,
                Vec::from([21]),
                [
                    // R13x43
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(14, Vec::from([ECB::new(1, 27)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(28, Vec::from([ECB::new(1, 13)])),
                ],
            ),
            Version::new(
                19,
                Vec::from([19, 39]),
                [
                    // R13x59
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(22, Vec::from([ECB::new(1, 38)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(20, Vec::from([ECB::new(2, 10)])),
                ],
            ),
            Version::new(
                20,
                Vec::from([25, 51]),
                [
                    // R13x77
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(16, Vec::from([ECB::new(1, 26), ECB::new(1, 27)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(28, Vec::from([ECB::new(1, 14), ECB::new(1, 15)])),
                ],
            ),
            Version::new(
                21,
                Vec::from([23, 49, 75]),
                [
                    // R13x99
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(20, Vec::from([ECB::new(1, 36), ECB::new(1, 37)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(26, Vec::from([ECB::new(1, 11), ECB::new(2, 12)])),
                ],
            ),
            Version::new(
                22,
                Vec::from([27, 55, 83, 111]),
                [
                    // R13x139
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(20, Vec::from([ECB::new(2, 35), ECB::new(1, 36)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(28, Vec::from([ECB::new(2, 13), ECB::new(2, 14)])),
                ],
            ),
            Version::new(
                23,
                Vec::from([21]),
                [
                    // R15x43
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(18, Vec::from([ECB::new(1, 33)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(18, Vec::from([ECB::new(1, 7), ECB::new(1, 8)])),
                ],
            ),
            Version::new(
                24,
                Vec::from([19, 39]),
                [
                    // R15x59
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(26, Vec::from([ECB::new(1, 48)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(24, Vec::from([ECB::new(2, 13)])),
                ],
            ),
            Version::new(
                25,
                Vec::from([25, 51]),
                [
                    // R15x77
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(18, Vec::from([ECB::new(1, 33), ECB::new(1, 34)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(24, Vec::from([ECB::new(2, 10), ECB::new(1, 11)])),
                ],
            ),
            Version::new(
                26,
                Vec::from([23, 49, 75]),
                [
                    // R15x99
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(24, Vec::from([ECB::new(2, 44)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(22, Vec::from([ECB::new(4, 12)])),
                ],
            ),
            Version::new(
                27,
                Vec::from([27, 55, 83, 111]),
                [
                    // R15x139
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(24, Vec::from([ECB::new(2, 42), ECB::new(1, 43)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(26, Vec::from([ECB::new(1, 13), ECB::new(4, 14)])),
                ],
            ),
            Version::new(
                28,
                Vec::from([21]),
                [
                    // R17x43
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(22, Vec::from([ECB::new(1, 39)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(20, Vec::from([ECB::new(1, 10), ECB::new(1, 11)])),
                ],
            ),
            Version::new(
                29,
                Vec::from([19, 39]),
                [
                    // R17x59
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(16, Vec::from([ECB::new(2, 28)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(30, Vec::from([ECB::new(2, 14)])),
                ],
            ),
            Version::new(
                30,
                Vec::from([25, 51]),
                [
                    // R17x77
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(22, Vec::from([ECB::new(2, 39)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(28, Vec::from([ECB::new(1, 12), ECB::new(2, 13)])),
                ],
            ),
            Version::new(
                31,
                Vec::from([23, 49, 75]),
                [
                    // R17x99
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(20, Vec::from([ECB::new(2, 33), ECB::new(1, 34)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(26, Vec::from([ECB::new(4, 14)])),
                ],
            ),
            Version::new(
                32,
                Vec::from([27, 55, 83, 111]),
                [
                    // R17x139
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(20, Vec::from([ECB::new(4, 38)])),
                    ECBlocks::new(0, Vec::from([ECB::new(0, 0)])),
                    ECBlocks::new(26, Vec::from([ECB::new(2, 12), ECB::new(4, 13)])),
                ],
            ),
        ])
    }
}

/*

/**
     * See ISO/IEC 23941:2022 Annex D, Table D.1 - Column coordinates of centre module of alignment patterns
     * See ISO/IEC 23941:2022 7.5.1, Table 8 - Error correction characteristics for rMQR
     */
    static const Version allVersions[] = {
        // Version number, alignment pattern centres, `ECBlocks`
        { 1, {21}, { // R7x43
            // 4 `ECBlocks`, one for each `ecLevel` - rMQR only uses M & H but using 2 dummies to keep `ecLevel` index same as QR Code
            // Each begins with no. of error correction codewords divided by no. of error correction blocks, followed by 2 `ECBlock`s
            // Each `ECBlock` begins with no. of error correction blocks followed by no. of data codewords per block
             0, 0,  0, 0,  0, // L (dummy) - also used to differentiate rMQR from Model2 in `Version::Version()`
             7, 1,  6, 0,  0, // M
             0, 0,  0, 0,  0, // Q (dummy)
            10, 1,  3, 0,  0, // H
            }},
        { 2, {19, 39}, { // R7x59
             0, 0,  0, 0,  0,
             9, 1, 12, 0,  0,
             0, 0,  0, 0,  0,
            14, 1,  7, 0,  0,
            }},
        { 3, {25, 51}, { // R7x77
             0, 0,  0, 0,  0,
            12, 1, 20, 0,  0,
             0, 0,  0, 0,  0,
            22, 1, 10, 0,  0,
            }},
        { 4, {23, 49, 75}, { // R7x99
             0, 0,  0, 0,  0,
            16, 1, 28, 0,  0,
             0, 0,  0, 0,  0,
            30, 1, 14, 0,  0,
            }},
        { 5, {27, 55, 83, 111}, { // R7x139
             0, 0,  0, 0,  0,
            24, 1, 44, 0,  0,
             0, 0,  0, 0,  0,
            22, 2, 12, 0,  0,
            }},
        { 6, {21}, { // R9x43
             0, 0,  0, 0,  0,
             9, 1, 12, 0,  0,
             0, 0,  0, 0,  0,
            14, 1,  7, 0,  0,
            }},
        { 7, {19, 39}, { // R9x59
             0, 0,  0, 0,  0,
            12, 1, 21, 0,  0,
             0, 0,  0, 0,  0,
            22, 1, 11, 0,  0,
            }},
        { 8, {25, 51}, { // R9x77
             0, 0,  0, 0,  0,
            18, 1, 31, 0,  0,
             0, 0,  0, 0,  0,
            16, 1,  8, 1,  9,
            }},
        { 9, {23, 49, 75}, { // R9x99
             0, 0,  0, 0,  0,
            24, 1, 42, 0,  0,
             0, 0,  0, 0,  0,
            22, 2, 11, 0,  0,
            }},
        {10, {27, 55, 83, 111}, { // R9x139
             0, 0,  0, 0,  0,
            18, 1, 31, 1, 32,
             0, 0,  0, 0,  0,
            22, 3, 11, 0,  0,
            }},
        {11, {}, { // R11x27
             0, 0,  0, 0,  0,
             8, 1,  7, 0,  0,
             0, 0,  0, 0,  0,
            10, 1,  5, 0,  0,
            }},
        {12, {21}, { // R11x43
             0, 0,  0, 0,  0,
            12, 1, 19, 0,  0,
             0, 0,  0, 0,  0,
            20, 1, 11, 0,  0,
            }},
        {13, {19, 39}, { // R11x59
             0, 0,  0, 0,  0,
            16, 1, 31, 0,  0,
             0, 0,  0, 0,  0,
            16, 1,  7, 1,  8,
            }},
        {14, {25, 51}, { // R11x77
             0, 0,  0, 0,  0,
            24, 1, 43, 0,  0,
             0, 0,  0, 0,  0,
            22, 1, 11, 1, 12,
            }},
        {15, {23, 49, 75}, { // R11x99
             0, 0,  0, 0,  0,
            16, 1, 28, 1, 29,
             0, 0,  0, 0,  0,
            30, 1, 14, 1, 15,
            }},
        {16, {27, 55, 83, 111}, { // R11x139
             0, 0,  0, 0,  0,
            24, 2, 42, 0,  0,
             0, 0,  0, 0,  0,
            30, 3, 14, 0,  0,
            }},
        {17, {}, { // R13x27
             0, 0,  0, 0,  0,
             9, 1, 12, 0,  0,
             0, 0,  0, 0,  0,
            14, 1,  7, 0,  0,
            }},
        {18, {21}, { // R13x43
             0, 0,  0, 0,  0,
            14, 1, 27, 0,  0,
             0, 0,  0, 0,  0,
            28, 1, 13, 0,  0,
            }},
        {19, {19, 39}, { // R13x59
             0, 0,  0, 0,  0,
            22, 1, 38, 0,  0,
             0, 0,  0, 0,  0,
            20, 2, 10, 0,  0,
            }},
        {20, {25, 51}, { // R13x77
             0, 0,  0, 0,  0,
            16, 1, 26, 1, 27,
             0, 0,  0, 0,  0,
            28, 1, 14, 1, 15,
            }},
        {21, {23, 49, 75}, { // R13x99
             0, 0,  0, 0,  0,
            20, 1, 36, 1, 37,
             0, 0,  0, 0,  0,
            26, 1, 11, 2, 12,
            }},
        {22, {27, 55, 83, 111}, { // R13x139
             0, 0,  0, 0,  0,
            20, 2, 35, 1, 36,
             0, 0,  0, 0,  0,
            28, 2, 13, 2, 14,
            }},
        {23, {21}, { // R15x43
             0, 0,  0, 0,  0,
            18, 1, 33, 0,  0,
             0, 0,  0, 0,  0,
            18, 1,  7, 1,  8,
            }},
        {24, {19, 39}, { // R15x59
             0, 0,  0, 0,  0,
            26, 1, 48, 0,  0,
             0, 0,  0, 0,  0,
            24, 2, 13, 0,  0,
            }},
        {25, {25, 51}, { // R15x77
             0, 0,  0, 0,  0,
            18, 1, 33, 1, 34,
             0, 0,  0, 0,  0,
            24, 2, 10, 1, 11,
            }},
        {26, {23, 49, 75}, { // R15x99
             0, 0,  0, 0,  0,
            24, 2, 44, 0,  0,
             0, 0,  0, 0,  0,
            22, 4, 12, 0,  0,
            }},
        {27, {27, 55, 83, 111}, { // R15x139
             0, 0,  0, 0,  0,
            24, 2, 42, 1, 43,
             0, 0,  0, 0,  0,
            26, 1, 13, 4, 14,
            }},
        {28, {21}, { // R17x43
             0, 0,  0, 0,  0,
            22, 1, 39, 0,  0,
             0, 0,  0, 0,  0,
            20, 1, 10, 1, 11,
            }},
        {29, {19, 39}, { // R17x59
             0, 0,  0, 0,  0,
            16, 2, 28, 0,  0,
             0, 0,  0, 0,  0,
            30, 2, 14, 0,  0,
            }},
        {30, {25, 51}, { // R17x77
             0, 0,  0, 0,  0,
            22, 2, 39, 0,  0,
             0, 0,  0, 0,  0,
            28, 1, 12, 2, 13,
            }},
        {31, {23, 49, 75}, { // R17x99
             0, 0,  0, 0,  0,
            20, 2, 33, 1, 34,
             0, 0,  0, 0,  0,
            26, 4, 14, 0,  0,
            }},
        {32, {27, 55, 83, 111}, { // R17x139
             0, 0,  0, 0,  0,
            20, 4, 38, 0,  0,
             0, 0,  0, 0,  0,
            26, 2, 12, 4, 13,
            }},
    };

    if (number < 1 || number > Size(allVersions))
        return nullptr;
    return allVersions + number - 1;

*/
