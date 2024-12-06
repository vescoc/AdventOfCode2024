fn main() {
    let model_props = ui::ModelProps {
        input: day20::INPUT.to_string(),
        solve_1: day20::solve_1,
        solve_2: day20::solve_2,
    };
    yew::Renderer::<ui::Model<_, _, _, _>>::with_props(model_props).render();
}
