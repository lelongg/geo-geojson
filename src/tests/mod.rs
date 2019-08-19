use crate::from_str;
use geo_types::{Coordinate, Geometry, GeometryCollection, LineString, Point, Polygon};
use once_cell::sync::Lazy;

static DEMO_JSON: Lazy<&str> = Lazy::new(|| include_str!("demo.json"));

#[test]
fn test_demo() {
    let expected = Ok(GeometryCollection(
        [
            Geometry::Polygon(Polygon::new(
                LineString(
                    [
                        Coordinate {
                            x: 114.184_663_295_745_86,
                            y: 22.267_890_315_905_507,
                        },
                        Coordinate {
                            x: 114.188_407_659_530_64_f64,
                            y: 22.269_260_470_261_78,
                        },
                        Coordinate {
                            x: 114.186_519_384_384_16,
                            y: 22.267_284_663_689_324,
                        },
                        Coordinate {
                            x: 114.187_624_454_498_3,
                            y: 22.265_437_904_683_4,
                        },
                        Coordinate {
                            x: 114.184_995_889_663_72,
                            y: 22.266_271_927_897_595,
                        },
                        Coordinate {
                            x: 114.184_663_295_745_86,
                            y: 22.267_890_315_905_507,
                        },
                    ]
                    .to_vec(),
                ),
                [].to_vec(),
            )),
            Geometry::LineString(LineString(
                [
                    Coordinate {
                        x: 114.188_364_744_186_42,
                        y: 22.270_749_753_256_833,
                    },
                    Coordinate {
                        x: 114.190_478_324_890_17,
                        y: 22.269_776_756_828_64_f64,
                    },
                ]
                .to_vec(),
            )),
            Geometry::LineString(LineString(
                [
                    Coordinate {
                        x: 114.189_255_237_579_36,
                        y: 22.267_592_454_487_456,
                    },
                    Coordinate {
                        x: 114.192_044_734_954_83,
                        y: 22.269_280_327_472_66,
                    },
                    Coordinate {
                        x: 114.191_218_614_578_26,
                        y: 22.264_881_886_445_78,
                    },
                    Coordinate {
                        x: 114.193_578_958_511_37,
                        y: 22.266_927_228_364_477,
                    },
                    Coordinate {
                        x: 114.189_351_797_103_9,
                        y: 22.266_530_076_932_69,
                    },
                ]
                .to_vec(),
            )),
            Geometry::Point(Point(Coordinate {
                x: 114.189_491_271_972_68,
                y: 22.271_802_170_346_884,
            })),
            Geometry::Polygon(Polygon::new(
                LineString(
                    [
                        Coordinate {
                            x: 114.186_465_740_203_86,
                            y: 22.263_918_778_199_81,
                        },
                        Coordinate {
                            x: 114.192_259_311_676_04,
                            y: 22.263_759_914_347_176,
                        },
                        Coordinate {
                            x: 114.192_452_430_725_13,
                            y: 22.260_046_420_421_386,
                        },
                        Coordinate {
                            x: 114.185_585_975_647,
                            y: 22.260_503_165_629_643,
                        },
                        Coordinate {
                            x: 114.186_465_740_203_86,
                            y: 22.263_918_778_199_81,
                        },
                    ]
                    .to_vec(),
                ),
                [LineString(
                    [
                        Coordinate {
                            x: 114.187_324_047_088_64_f64,
                            y: 22.263_243_605_580_74,
                        },
                        Coordinate {
                            x: 114.187_388_420_105,
                            y: 22.261_218_068_181,
                        },
                        Coordinate {
                            x: 114.190_928_936_004_64_f64,
                            y: 22.260_959_909_347_545,
                        },
                        Coordinate {
                            x: 114.191_014_766_693_12,
                            y: 22.262_687_578_625_24,
                        },
                        Coordinate {
                            x: 114.187_324_047_088_64_f64,
                            y: 22.263_243_605_580_74,
                        },
                    ]
                    .to_vec(),
                )]
                .to_vec(),
            )),
        ]
        .to_vec(),
    ));

    let result = from_str::<f64>(&DEMO_JSON);
    assert_eq!(expected, result);
}
