use std::collections::HashMap;
use crate::core::model::abstract_syntax_tree_nodes::assignable::Assignable;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::expression::Expression;
use crate::core::model::abstract_syntax_tree_nodes::assignables::equation_parser::operator::Operator;
use crate::core::model::abstract_syntax_tree_nodes::identifier::Identifier;
use crate::core::model::types::float::FloatType;
use crate::core::model::types::integer::IntegerType;
use crate::core::model::types::mutability::Mutability;
use crate::core::model::types::ty::Type;
use crate::core::optimization::optimization_trait::{ConstFoldable, Optimization, OptimizationContext};
use crate::core::parser::static_type_context::StaticTypeContext;
use crate::core::parser::types::boolean::Boolean;

impl Optimization for Expression {
    fn o1(&mut self, static_type_context: &mut StaticTypeContext, optimization: OptimizationContext) -> OptimizationContext {
        let folded = self.const_fold_assignment(static_type_context, &optimization);
        if let Some(folded) = folded {
            *self = Expression {
                value: Some(Box::new(folded)),
                lhs: None,
                rhs: None,
                operator: Operator::Noop,
                prefix_arithmetic: None,
                index_operator: None,
                positive: true,
            };
        }

        optimization
    }
}

impl ConstFoldable for Expression {
    fn is_const(&self) -> bool {
        if let (Some(value), None, None, None) = (&self.value, &self.lhs, &self.rhs, &self.prefix_arithmetic) {
            return value.is_const();
        }

        if let (Some(left), Some(right), _) = (&self.lhs, &self.rhs, self.operator) {
            return left.is_const() && right.is_const();
        }

        false
    }

    fn const_fold(&self, static_type_context: &StaticTypeContext, optimization_context: &OptimizationContext) -> Option<Self> {
        if let Some(folded) = self.const_fold_assignment(static_type_context, optimization_context) {
            return Some(Expression {
                value: Some(Box::new(folded)),
                lhs: None,
                rhs: None,
                operator: Operator::Noop,
                prefix_arithmetic: None,
                index_operator: None,
                positive: true,
            });
        }

        Some(self.clone())
    }
}

impl Expression {
    fn const_fold_assignment(&self, static_type_context: &StaticTypeContext, optimization_context: &OptimizationContext) -> Option<Assignable> {
        if let (Some(value), None, None, None) = (&self.value, &self.lhs, &self.rhs, &self.prefix_arithmetic) {
            return Some(*value.clone());
        }

        if let (Some(left), Some(right), operation) = (&self.lhs, &self.rhs, self.operator) {
            let left = left.const_fold_assignment(static_type_context, optimization_context);
            let right = right.const_fold_assignment(static_type_context, optimization_context);

            let left = match &left {
                Some(Assignable::Identifier(identifier)) => {
                    if let Some(const_variable_assignment) = optimization_context.constant_variables.get(&identifier.name) {
                        Some(const_variable_assignment.clone())
                    } else {
                        left
                    }
                }
                Some(Assignable::MethodCall(method_call)) => {
                    if let Some(const_method_assignment) = optimization_context.const_method_definitions.get(&method_call.method_label_name(static_type_context)) {
                        Some(const_method_assignment.clone())
                    } else {
                        left
                    }
                }
                _ => left,
            };

            let right = match &right {
                Some(Assignable::Identifier(identifier)) => {
                    if let Some(const_variable_assignment) = optimization_context.constant_variables.get(&identifier.name) {
                        Some(const_variable_assignment.clone())
                    } else {
                        right
                    }
                }
                Some(Assignable::MethodCall(method_call)) => {
                    if let Some(const_method_assignment) = optimization_context.const_method_definitions.get(&method_call.method_label_name(static_type_context)) {
                        Some(const_method_assignment.clone())
                    } else {
                        right
                    }
                }
                _ => right,
            };



            if left.is_none() || right.is_none() {
                return None;
            }

            if let (Some(left), Some(right)) = (left, right) {
                let lhs_type = left.get_type(static_type_context)?;
                let rhs_type = right.get_type(static_type_context)?;

                let mut base_type_matrix: HashMap<(Type, Operator, Type), Type> = HashMap::new();

                base_type_matrix.insert((Type::Custom(Identifier { name: "string".to_string() }, Mutability::Immutable), Operator::Add, Type::Custom(Identifier { name: "string".to_string() }, Mutability::Immutable)), Type::Custom(Identifier { name: "*string".to_string() }, Mutability::Immutable));

                IntegerType::operation_matrix(&mut base_type_matrix);
                FloatType::operation_matrix(&mut base_type_matrix);
                Boolean::operation_matrix(&mut base_type_matrix);

                base_type_matrix.get(&(lhs_type, operation, rhs_type))?;

                return match operation {
                    Operator::Add => left.add(right, static_type_context),
                    Operator::Sub => left.sub(right, static_type_context),
                    Operator::Mul => left.mul(right, static_type_context),
                    Operator::Div => left.div(right, static_type_context),
                    _ => None
                }
            }
        }
        None
    }
}
