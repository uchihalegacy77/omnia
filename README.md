Omnia
​The Universal, Zero-Bloat, High-Velocity Programming Language.
​Infinite logic. Zero wasted space.
​Omnia is a multi-paradigm, statically inferred programming language designed for the modern era. It combines the instant feedback of an interpreted language with the indestructible, storage-friendly speed of raw machine code.
​Whether you are building a microscopic embedded system or a massive web server, Omnia is designed to run everywhere, instantly.
​🚀 Core Architecture
​The Omni-Drive Engine: Why choose between a compiler and an interpreter? Omnia fuses both. Test your code instantly with the built-in REPL, use comptime to bake heavy calculations during the build, and compile down to pure, microscopic machine code for production.
​Neutrino Fibers: Concurrency without the crash. Launch millions of microscopic background tasks using the spawn keyword. If one fiber fails, it safely returns an Err value without taking down your system.
​Orbit Package Manager: Say goodbye to node_modules and bloated hard drives. Orbit imports libraries directly via secure URLs, caches them globally once, and aggressively tree-shakes unused code from your final app.
​The "Event Horizon" Philosophy: Write code that is safe, strictly typed, and beautiful. Built-in pattern matching and the ? error-handling operator keep your logic perfectly clean.
// Import the HTTP library directly from the universe
import { Server } from "https://omnia.pkg/net/v1";

fn main() {
    mut server = Server.new(8080);
    print("Pulsar Engine ignited. Listening on port 8080...");
    
    server.listen(|req| {
        // Spawn a lightweight Neutrino fiber for every single visitor.
        spawn handle_traffic(req);
    });
}

fn handle_traffic(req: Request) -> Result<Response, Error> {
    match req.path {
        "/" => Ok(Response.text("Welcome to the Event Horizon.")),
        _   => Err(Error.NotFound)
    }
}
curl -sL https://omnia-lang.org/install.sh | bash
# Build a native, tiny binary for Windows
omnia build app.om --target windows-x64

# Build an ultra-optimized version for Android
omnia build app.om --target android-arm

# Build for the web
omnia build app.om --target wasm
MIT License
​Copyright (c) 2026 The Omnia Foundation
​Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
​The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
​THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
