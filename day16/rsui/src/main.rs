fn main() {
    let model_props = ui::ModelProps {
        input: day16::INPUT.to_string(),
        solve_1: day16::solve_1,
        solve_2: day16::solve_2,
    };
    yew::Renderer::<ui::Model<_, _, _, _>>::with_props(model_props).render();
}
