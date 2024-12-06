fn main() {
    let model_props = ui::ModelProps {
        input: day21::INPUT.to_string(),
        solve_1: day21::solve_1,
        solve_2: day21::solve_2,
    };
    yew::Renderer::<ui::Model<_, _, _, _>>::with_props(model_props).render();
}
