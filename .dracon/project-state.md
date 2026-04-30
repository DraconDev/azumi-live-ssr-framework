# Project State

## Current Focus
Refactor Live macro to inline generate prediction and field arrays using token quoting

## Completed
- [x] Replace `predictions_const` collection with quoted tuple generation via `map(|(method, dsl)| quote! { (#method, #dsl) })`.
- [x] Update `__AZUMI_PREDICTIONS` const expansion to use `#(predictions_const),*`.
- [x] Replace static field references with inline slice construction using `#(#local_field_names_static),*`.
