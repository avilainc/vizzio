//! Custom allocators and memory management

use avila_atom::pool::ObjectPool;

#[derive(Debug)]
struct GameObject {
    id: usize,
    position: (f32, f32),
    velocity: (f32, f32),
    active: bool,
}

impl GameObject {
    fn new(id: usize) -> Self {
        Self {
            id,
            position: (0.0, 0.0),
            velocity: (1.0, 1.0),
            active: true,
        }
    }

    fn update(&mut self, dt: f32) {
        if self.active {
            self.position.0 += self.velocity.0 * dt;
            self.position.1 += self.velocity.1 * dt;
        }
    }
}

fn main() {
    println!("=== Object Pool Example ===\n");

    // Create pool for game objects
    let mut pool = ObjectPool::with_capacity(100);

    // Spawn objects
    let mut object_ids = Vec::new();
    for i in 0..50 {
        let id = pool.acquire(|| GameObject::new(i));
        object_ids.push(id);
    }

    println!("Spawned {} game objects", object_ids.len());

    // Update objects
    let dt = 0.016; // 16ms frame time
    for _ in 0..10 {
        for &id in &object_ids {
            if let Some(obj) = pool.get_mut(id) {
                obj.update(dt);
            }
        }
    }

    // Display some objects
    println!("\nObject states after 10 frames:");
    for &id in object_ids.iter().take(3) {
        if let Some(obj) = pool.get(id) {
            println!("  Object {}: pos=({:.2}, {:.2})",
                     obj.id, obj.position.0, obj.position.1);
        }
    }

    // Despawn half the objects
    for &id in object_ids.iter().take(25) {
        pool.release(id);
    }

    println!("\nDespawned 25 objects");

    // Spawn new objects (reuses slots)
    for i in 50..60 {
        let id = pool.acquire(|| GameObject::new(i));
        println!("  Spawned object {} in slot {}", i, id);
    }

    println!("\n✓ Object pool allows O(1) alloc/dealloc!");
}
