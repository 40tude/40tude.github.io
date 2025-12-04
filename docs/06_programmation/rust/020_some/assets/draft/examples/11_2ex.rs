// cargo run --example 11_2ex
// RAII pattern using take() - Resource is automatically released when Guard is dropped

struct Resource {
    name: String,
}

impl Resource {
    fn new(name: &str) -> Self {
        println!("\t[{}] Acquired", name);
        Self {
            name: name.to_string(),
        }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("\t[{}] Released", self.name);
    }
}

struct Guard {
    resource: Option<Resource>,
}

impl Guard {
    fn new(name: &str) -> Self {
        Self {
            resource: Some(Resource::new(name)),
        }
    }

    // Manually release before scope ends
    fn release(&mut self) {
        if let Some(r) = self.resource.take() {
            println!("\t[{}] Early release", r.name);
        } // r is dropped here
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        if self.resource.is_some() {
            println!("\tGuard dropped with resource still held");
        }
        // resource.take() not needed here - Option<T> drops automatically
    }
}

fn main() {
    println!("Example 1: Auto release at scope end");
    {
        let _guard = Guard::new("DB Connection");
        println!("\tDoing some work...");
    } // _guard dropped here, Resource released

    println!("\nExample 2: Early release with take()");
    {
        let mut guard = Guard::new("File Handle");
        println!("\tDoing work...");
        guard.release(); // Release early via take()
        println!("\tMore work after release...");
    } // Guard dropped, but resource already gone
}
