/*
 * prifri, 2022.10.05:
 * - piston_window
 *   피스톤 게임 엔진의 핵심 기능에 대한 래퍼를 제공하여 화면에 대상을 쉽게
 *   그릴수 있게 한다.
 * - piston2d-graphics
 *   벡터 대수를 제공하며 움직임을 시뮬레이트하는데 중요하다.
 */

/*
 * prifri, 2022.10.05:
 * - Vec2d
 *   이차원 벡터에 대한 연산과 변환 기능을 제공
 * - piston_window
 *   GUI 프로그램을 만들고 그 위에 모양을 그리는 도구를 제공.
 */
use graphics::math::{Vec2d, add, mul_scalar};
use piston_window::*;
use rand::prelude::*;

/*
 * prifri, 2022.10.05:
 * - 메모리 할당을 제하는 기능을 제공
 */
use std::alloc::{GlobalAlloc, System, Layout};

/*
 * prifri, 2022.10.05:
 * - 시스템 시계에 접근하는 기능 제공.
 */
use std::time::Instant;

/*
 * prifri, 2022.10.05:
 * - global_allocator
 *   다음에 나오는 값 ALLOCATOR가 GlobalAlloc trait를 충족하도록 한다.
 */
#[global_allocator]
static ALLOCATOR: ReportingAllocator = ReportingAllocator;
struct ReportingAllocator;

unsafe impl GlobalAlloc for ReportingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let start = Instant::now();
        let ptr = System.alloc(layout);
        let end = Instant::now();
        let time_taken = end - start;
        let bytes_requested = layout.size();
        eprintln!("{}\t{}", bytes_requested, time_taken.as_nanos());
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }
}

struct Particle {
    height: f64,
    width: f64,
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
    acceleration: Vec2d<f64>,
    color: [f32; 4],
}

struct World {
    current_turn: u64,
    particles: Vec<Box<Particle>>,
    height: f64,
    width: f64,
    rng: ThreadRng,
}

impl Particle {
    fn new(world: &World) -> Particle {
        let mut rng = thread_rng();
        let x = rng.gen_range(0.0..=world.width);
        let y = world.height;
        let x_velocity = 0.0;
        let y_velocity = rng.gen_range(-2.0..0.0);
        let x_acceleration = 0.0;
        let y_acceleration = rng.gen_range(0.0..0.15);

        Particle {
            height: 4.0,
            width: 4.0,
            position: [x, y].into(),
            velocity: [x_velocity, y_velocity].into(),
            acceleration: [x_acceleration, y_acceleration].into(),
            color: [1.0, 1.0, 1.0, 0.99],
        }
    }

    fn update(&mut self) {
        self.velocity = add(self.velocity,
                            self.acceleration);
        self.position = add(self.position,
                            self.velocity);
        self.acceleration = mul_scalar(
            self.acceleration,
            0.7
            );
        self.color[3] *= 0.995;
    }
}

impl World {
    fn new(width: f64, height: f64) -> World {
        World {
            current_turn: 0,
            particles: Vec::<Box<Particle>>::new(),
            height,
            width,
            rng: thread_rng(),
        }
    }

    fn add_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            let particle = Particle::new(&self);
            let boxed_particle = Box::new(particle);
            self.particles.push(boxed_particle);
        }
    }

    fn remove_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            let mut to_delete = None;
            let particle_iter = self.particles
                .iter()
                .enumerate();

            for (i, particle) in particle_iter {
                if particle.color[3] < 0.02 {
                    to_delete = Some(i);
                }
                break;
            }

            if let Some(i) = to_delete {
                self.particles.remove(i);
            } else {
                self.particles.remove(0);
            }
        }
    }

    fn update(&mut self) {
        let n = self.rng.gen_range(-3..=3);

        if n > 0 {
            self.add_shapes(n);
        } else {
            self.remove_shapes(n);
        }

        self.particles.shrink_to_fit();
        for shape in &mut self.particles {
            shape.update();
        }
        self.current_turn += 1;
    }
}


/*
 * prifri, 2022.10.05:
 * - vsync 에러땜에 test. vsync(true)넣으면 안나온다.
 */
fn test() {
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .vsync(true)
        .exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      context.transform,
                      graphics);
        });
    }
}

fn test2() {
    let (width, height) = (1280.0, 960.0);
    let mut window: PistonWindow = WindowSettings::new(
        "particles", [width, height]
        ).vsync(true)
        .exit_on_esc(true)
        .build().unwrap();
        //.expect("Could not create a window.");

    let mut world = World::new(width, height);
    world.add_shapes(1000);

    while let Some(event) = window.next() {
        world.update();

/*
 * prifri, 2022.10.05:
 * - | .. |
 *   클로저. 람다 or 인라인 함수등으로 불림.
 */
        window.draw_2d(&event, |ctx, renderer, _device| {
            clear([0.15, 0.17, 0.17, 0.9], renderer);

            for s in &mut world.particles {
                let size = [s.position[0], s.position[1], s.width, s.height];
                rectangle(s.color, size, ctx.transform, renderer);
            }
        });
    }
}
fn main() {
    test2();
}
