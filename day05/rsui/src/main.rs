fn main() {
    let model_props = ui::ModelProps {
        input: day05::INPUT.to_string(),
        solve_1: day05::solve_1,
        solve_2: day05::solve_2,
    };
    yew::Renderer::<ui::Model<_, _, _, _>>::with_props(model_props).render();
}
