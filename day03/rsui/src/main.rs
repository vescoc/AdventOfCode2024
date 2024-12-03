fn main() {
    let model_props = ui::ModelProps {
        input: day03::INPUT.to_string(),
        solve_1: day03::solve_1,
        solve_2: day03::solve_2,
    };
    yew::Renderer::<ui::Model<_, _, _, _>>::with_props(model_props).render();
}
