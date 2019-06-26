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
                            x: 114.18466329574586,
                            y: 22.267890315905507,
                        },
                        Coordinate {
                            x: 114.18840765953064,
                            y: 22.26926047026178,
                        },
                        Coordinate {
                            x: 114.18651938438416,
                            y: 22.267284663689324,
                        },
                        Coordinate {
                            x: 114.1876244544983,
                            y: 22.2654379046834,
                        },
                        Coordinate {
                            x: 114.18499588966372,
                            y: 22.266271927897595,
                        },
                        Coordinate {
                            x: 114.18466329574586,
                            y: 22.267890315905507,
                        },
                    ]
                    .to_vec(),
                ),
                [].to_vec(),
            )),
            Geometry::LineString(LineString(
                [
                    Coordinate {
                        x: 114.18836474418642,
                        y: 22.270749753256833,
                    },
                    Coordinate {
                        x: 114.19047832489017,
                        y: 22.26977675682864,
                    },
                ]
                .to_vec(),
            )),
            Geometry::LineString(LineString(
                [
                    Coordinate {
                        x: 114.18925523757936,
                        y: 22.267592454487456,
                    },
                    Coordinate {
                        x: 114.19204473495483,
                        y: 22.26928032747266,
                    },
                    Coordinate {
                        x: 114.19121861457826,
                        y: 22.26488188644578,
                    },
                    Coordinate {
                        x: 114.19357895851137,
                        y: 22.266927228364477,
                    },
                    Coordinate {
                        x: 114.1893517971039,
                        y: 22.26653007693269,
                    },
                ]
                .to_vec(),
            )),
            Geometry::Point(Point(Coordinate {
                x: 114.18949127197268,
                y: 22.271802170346884,
            })),
            Geometry::Polygon(Polygon::new(
                LineString(
                    [
                        Coordinate {
                            x: 114.18646574020386,
                            y: 22.26391877819981,
                        },
                        Coordinate {
                            x: 114.19225931167604,
                            y: 22.263759914347176,
                        },
                        Coordinate {
                            x: 114.19245243072513,
                            y: 22.260046420421386,
                        },
                        Coordinate {
                            x: 114.185585975647,
                            y: 22.260503165629643,
                        },
                        Coordinate {
                            x: 114.18646574020386,
                            y: 22.26391877819981,
                        },
                    ]
                    .to_vec(),
                ),
                [LineString(
                    [
                        Coordinate {
                            x: 114.18732404708864,
                            y: 22.26324360558074,
                        },
                        Coordinate {
                            x: 114.187388420105,
                            y: 22.261218068181,
                        },
                        Coordinate {
                            x: 114.19092893600464,
                            y: 22.260959909347545,
                        },
                        Coordinate {
                            x: 114.19101476669312,
                            y: 22.26268757862524,
                        },
                        Coordinate {
                            x: 114.18732404708864,
                            y: 22.26324360558074,
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
