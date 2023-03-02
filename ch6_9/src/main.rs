//provides math operation ans conversion functionality for 2d vectors
use graphics::math::{Vec2d, add, mul_scalar};
//tools to create GUI and draw shapes
use piston_window::*;
// provides random numbers
use rand::prelude::*;
// provides facilities for controlling memory allocation
use std::alloc::{GlobalAlloc, System, Layout};
// provides acccess to the system clock
use std::time::Instant;

// marks the following value (ALLOCATOR) as satisfying the GlobalAlloc trait
#[global_allocator]
static ALLOCATOR: ReportingAllocator = ReportingAllocator;

// prints the time taken for each allocation to stdout as the program runs.
// provides a fairly accurate indication of the time taken for dynamic memory allocation
struct ReportingAllocator;

unsafe impl GlobalAlloc for ReportingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let start = Instant::now();
        //defers the actual memory allocation to the system default memory allocator
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

// contains the data that is useful for the lifetime of the program
struct World {
    current_turn: u64,
    particles: Vec<Box<Particle>>,
    height: f64,
    width: f64,
    rng: ThreadRng
}

//defines an object in 2d space
struct Particle {
    heigth: f64,
    width: f64,
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
    accelaration: Vec2d<f64>,
    color: [f32; 4],
}

impl Particle {
    fn new(world: &World) -> Particle {
        let mut rng = thread_rng();
        //starts a random position along the bottom of the window
        let x = rng.gen_range(0.0..=world.width);
        let y = world.height;
        let x_velocity = 0.0;
        //rises vertically over time
        let y_velocity = rng.gen_range(-2.0..0.0);
        let x_acceleration = 0.0;
        let y_accelaration = rng.gen_range(0.0..0.15);

        Particle { 
            heigth: 4.0,
            width: 4.0,
            position: [x, y].into(), //into converts the array of type [f64; 2] into Vec2d
            velocity: [x_velocity, y_velocity].into(),
            accelaration: [x_acceleration, y_accelaration].into(), 
            color: [1.0, 1.0, 1.0, 0.99], // inserts a saturated white that has a tiny amount of transparency
        }
    }

    fn update(&mut self) {
        //moves particle to its next position
        self.velocity = add(self.velocity, self.accelaration);
        self.position = add(self.position, self.velocity);
        //slows down the particle rate of increase as it travels across the screen
        self.accelaration = mul_scalar(self.accelaration, 0.7);
        // makes the particle more transparent over time
        self.color[3] *= 0.995;
    }
}

impl World {
    fn new(width : f64, height: f64) -> World {
        World { 
            current_turn: 0,
            // uses Box<Particle> rather than Particle to incur an extra memory allocation when every particle is created
            particles: Vec::<Box<Particle>>::new(),
            height: height,
            width: width,
            rng: thread_rng(),
        }
    }

    fn add_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            //creates a particle as a local variable on the stack
            let particle = Particle::new(&self);
            //takes ownership of particle moving its data to the heap
            // and creates a reference to that data on the stack
            let boxed_particle = Box::new(particle);
            self.particles.push(boxed_particle); // push to self.shapes
        }
    }

    fn remove_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            let mut to_delete = None;
            // particle_iter split into its own variable to more easily fit on the page
            let particle_iter = self.particles.iter().enumerate();

            //for n iteration, removes the first particle thats invisible. if there are no invisibles, then removes the oldest
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
            };
        }
    }

    fn update(&mut self) {
        let n = self.rng.gen_range(-3..=3);// returns a random between the values

        if n > 0 {
            self.add_shapes(n);
        } else {
            self.remove_shapes(n);
        }

        // ????????
        self.particles.shrink_to_fit();
        for shape in &mut self.particles {
            shape.update();
        }
        self.current_turn += 1;
    }

}
fn main() {
    let (width, height) = (1280.0, 960.0);
    let mut window: PistonWindow = WindowSettings::new("particles", [width, height])
    .exit_on_esc(true)
    .build().expect("Could not create a window.");

    let mut world = World::new(width, height);
    world.add_shapes(1000);

    while let Some(event) = window.next() {
        world.update();

        window.draw_2d(&event, | ctx, renderer, _device | {
            clear([0.15, 0.17, 0.17, 0.9], renderer);

            for s in &mut world.particles {
                let size = [s.position[0], s.position[1], s.width, s.heigth];
                rectangle(s.color, size, ctx.transform, renderer);
            }
        });
    }
}
