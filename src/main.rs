pub mod dcel;
pub mod lines;
mod randwalk;

use dcel::GeometricGraph;
use dcel::HalfEdgeId;
use nannou::{
    prelude::*,
    rand::{thread_rng, Rng},
};
use randwalk::MultiOscillator;

const DEBUG_HALF_EDGE_OFFSET: f32 = 3.0f32;
const DEBUG_EDGE_LENGTH: f32 = 0.9f32;

struct Model {
    graph: GeometricGraph<VertexData>,
    edge: HalfEdgeId,
    i: usize,       // TODO: remove
    was_twin: bool, // TODO: remove
}

struct VertexData {
    original_position: Vec2,
    noise: MultiOscillator<7>,
}

impl VertexData {
    fn rand_new<R: Rng>(original_position: Point2, rng: &mut R) -> Self {
        Self {
            original_position,
            noise: MultiOscillator::rand_new(rng),
        }
    }
}

fn create_default_example_graph() -> GeometricGraph<VertexData> {
    let mut this = GeometricGraph::new();

    let mut rng = thread_rng();

    let a = this.add_vertex(
        Point2::new(-50f32, -50f32),
        VertexData::rand_new(Point2::new(-50f32, -50f32), &mut rng),
    );
    let b = this.add_vertex(
        Point2::new(50f32, -50f32),
        VertexData::rand_new(Point2::new(50f32, -50f32), &mut rng),
    );
    let c = this.add_vertex(
        Point2::new(50f32, 50f32),
        VertexData::rand_new(Point2::new(50f32, 50f32), &mut rng),
    );
    let d = this.add_vertex(
        Point2::new(-50f32, 50f32),
        VertexData::rand_new(Point2::new(-50f32, 50f32), &mut rng),
    );

    this.add_edge(a, b);
    this.add_edge(b, c);
    this.add_edge(c, d);
    this.add_edge(d, a);

    this
}

impl Model {
    fn update(&mut self, _: &App, update: Update) {
        let time = update.since_start.as_secs_f32();

        for vertex in self.graph.iter_mut_vertices() {
            vertex.pos = vertex.data.original_position; //+ vertex.data.noise.generate(time);
        }
    }

    fn draw_to(&self, draw: &Draw) {
        self.graph
            .iter_edges()
            .map(|edge| (self.graph.origin(edge).pos, self.graph.target(edge).pos))
            .for_each(|(origin, target)| {
                draw.line().start(origin).end(target).finish();
            });
        // draw.polygon()
        //     .color(rgba(5u8, 250u8, 25u8, 76u8))
        //     .points(self.graph.iter_vertices().map(Vertex::final_pos))
        //     .finish();

        self.graph
            .iter_vertices()
            .map(|vertex| vertex.pos)
            .for_each(|pos| {
                draw.ellipse()
                    .color(BLACK)
                    .x(pos.x)
                    .y(pos.y)
                    .w(7f32)
                    .h(7f32)
                    .finish();
            });
    }
}

fn main() {
    nannou::app(model)
        .update(update)
        .event(event)
        .simple_window(view)
        .run();
}

fn event(app: &App, model: &mut Model, event: Event) {
    if let Event::WindowEvent {
        id: _,
        simple: Some(MousePressed(MouseButton::Left)),
    } = event
    {
        let pos = app.mouse.position();
        let other = model.graph.iter_vertices().last().unwrap().id(); // TODO: temp
        let vertex = model
            .graph
            .add_vertex(pos, VertexData::rand_new(pos, &mut thread_rng()));

        model.graph.add_edge(other, vertex); // TODO: temp
    }
}

fn model(_: &App) -> Model {
    let graph = create_default_example_graph();
    let edge = graph.iter_edges().next().unwrap().half_edge();
    Model {
        graph,
        i: 0,
        edge,
        was_twin: false,
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    model.update(app, update);

    model.i += 1;
    if model.i % 100 == 0 && model.graph.iter_edges().count() > 0 {
        if model.was_twin || model.i % 7 != 0 {
            model.edge = model.graph.half_edge(model.edge).next;
        } else {
            model.edge = model.graph.half_edge(model.edge).twin;
        }

        model.was_twin = !model.was_twin;
    }

    if model.i % 1234 == 0 {
        model
            .graph
            .remove_edge(model.graph.iter_edges().next().unwrap().id());
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(rgb(100u8, 100u8, 100u8));

    // model.draw_to(&draw);
    debug_draw(&draw, model); // TODO: remove debug

    draw.to_frame(app, &frame).unwrap();
}

fn debug_draw(draw: &Draw, model: &Model) {
    let graph = &model.graph;

    graph
        .iter_edges()
        .flat_map(|edge| {
            [
                graph.half_edge(edge.half_edge()),
                graph.half_edge(edge.twin_half_edge()),
            ]
        })
        .for_each(|edge| {
            let origin = graph.vertex(edge.origin());
            let target = graph.vertex(edge.target());
            let normal = (target.pos - origin.pos).perp().normalize();

            let mut arrow = draw
                .arrow()
                .weight(2f32)
                .start(
                    (1f32 - DEBUG_EDGE_LENGTH) * target.pos + DEBUG_EDGE_LENGTH * origin.pos
                        - normal * DEBUG_HALF_EDGE_OFFSET,
                )
                .end(
                    DEBUG_EDGE_LENGTH * target.pos + (1f32 - DEBUG_EDGE_LENGTH) * origin.pos
                        - normal * DEBUG_HALF_EDGE_OFFSET,
                );

            if model.edge == edge.id() {
                arrow = arrow.color(RED);
            }

            arrow.finish();
        });

    graph
        .iter_vertices()
        .for_each(|vertex| draw.ellipse().xy(vertex.pos).w(10f32).h(10f32).finish());
}
