//! PostGIS extension for sea-query with support for the `postgis::ewkb::Geometry` type
//!
//! This module adds support for working with PostGIS spatial data types in sea-query.
//! It provides implementations for converting between PostGIS `Geometry` types and
//! sea-query's `Value` type, as well as utilities for working with PostGIS functions.

use crate::{Alias, ColumnType, IntoIden, Value, ValueType, ValueTypeErr};
use postgis::ewkb::{AsEwkbGeometry, EwkbRead, EwkbWrite, GeometryT, Point};

/// Implement ValueType for PostGIS Geometry
impl ValueType for GeometryT<Point> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::Bytes(Some(b)) => {
                GeometryT::<Point>::read_ewkb(&mut b.as_slice()).map_err(|_| ValueTypeErr)
            }
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        "Geometry".to_owned()
    }

    fn column_type() -> ColumnType {
        ColumnType::Custom(Alias::new("geometry").into_iden())
    }

    fn array_type() -> crate::ArrayType {
        crate::ArrayType::Bytes
    }
}

/// Implement From<GeometryT<Point>> for Value
impl From<GeometryT<Point>> for Value {
    fn from(geom: GeometryT<Point>) -> Self {
        let mut b = Vec::<u8>::new();
        geom.as_ewkb().write_ewkb(&mut b).unwrap();
        Value::Bytes(Some(Box::new(b)))
    }
}

// Allow GeometryT<Point> as a value parameter
// impl crate::IntoValueTuple for GeometryT<Point> {
//     fn into_value_tuple(self) -> (Value, ValueType) {
//         (self.into(), ValueType::Custom("Geometry".into()))
//     }
// }

// Extension trait for PostgresQueryBuilder to handle geometry-specific functions
// pub trait PostgresGeometryExt {
//     /// Creates a ST_Distance function call
//     fn st_distance<T, U>(&self, geom1: T, geom2: U) -> SimpleExpr
//     where
//         T: Into<SimpleExpr>,
//         U: Into<SimpleExpr>;

//     /// Creates a ST_Contains function call
//     fn st_contains<T, U>(&self, geom1: T, geom2: U) -> SimpleExpr
//     where
//         T: Into<SimpleExpr>,
//         U: Into<SimpleExpr>;

//     /// Creates a ST_DWithin function call
//     fn st_dwithin<T, U, D>(&self, geom1: T, geom2: U, distance: D) -> SimpleExpr
//     where
//         T: Into<SimpleExpr>,
//         U: Into<SimpleExpr>,
//         D: Into<SimpleExpr>;

//     /// Creates a ST_Transform function call
//     fn st_transform<G, S>(&self, geom: G, srid: S) -> SimpleExpr
//     where
//         G: Into<SimpleExpr>,
//         S: Into<SimpleExpr>;

//     /// Creates a ST_AsText function call
//     fn st_as_text<G>(&self, geom: G) -> SimpleExpr
//     where
//         G: Into<SimpleExpr>;

//     /// Creates a ST_GeomFromText function call
//     fn st_geom_from_text<T, S>(&self, text: T, srid: Option<S>) -> SimpleExpr
//     where
//         T: Into<SimpleExpr>,
//         S: Into<SimpleExpr>;
// }

// impl PostgresGeometryExt for crate::PostgresQueryBuilder {
//     fn st_distance<T, U>(&self, geom1: T, geom2: U) -> SimpleExpr
//     where
//         T: Into<SimpleExpr>,
//         U: Into<SimpleExpr>,
//     {
//         let func = Func::cust("ST_Distance".into_iden());
//         SimpleExpr::FunctionCall(func, vec![geom1.into(), geom2.into()])
//     }

//     fn st_contains<T, U>(&self, geom1: T, geom2: U) -> SimpleExpr
//     where
//         T: Into<SimpleExpr>,
//         U: Into<SimpleExpr>,
//     {
//         let func = Func::cust("ST_Contains");
//         SimpleExpr::FunctionCall(func, vec![geom1.into(), geom2.into()])
//     }

//     fn st_dwithin<T, U, D>(&self, geom1: T, geom2: U, distance: D) -> SimpleExpr
//     where
//         T: Into<SimpleExpr>,
//         U: Into<SimpleExpr>,
//         D: Into<SimpleExpr>,
//     {
//         let func = Func::cust("ST_DWithin");
//         SimpleExpr::FunctionCall(func, vec![geom1.into(), geom2.into(), distance.into()])
//     }

//     fn st_transform<G, S>(&self, geom: G, srid: S) -> SimpleExpr
//     where
//         G: Into<SimpleExpr>,
//         S: Into<SimpleExpr>,
//     {
//         let func = Func::cust("ST_Transform");
//         SimpleExpr::FunctionCall(func, vec![geom.into(), srid.into()])
//     }

//     fn st_as_text<G>(&self, geom: G) -> SimpleExpr
//     where
//         G: Into<SimpleExpr>,
//     {
//         let func = Func::cust("ST_AsText");
//         SimpleExpr::FunctionCall(func, vec![geom.into()])
//     }

//     fn st_geom_from_text<T, S>(&self, text: T, srid: Option<S>) -> SimpleExpr
//     where
//         T: Into<SimpleExpr>,
//         S: Into<SimpleExpr>,
//     {
//         let func = Func::cust("ST_GeomFromText");
//         let mut params = vec![text.into()];
//         if let Some(srid) = srid {
//             params.push(srid.into());
//         }
//         SimpleExpr::FunctionCall(func, params)
//     }
// }
