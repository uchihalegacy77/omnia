🌌 Omnia
Natural Flow. Rigid Structure. Immutable Core.
Omnia balances the human need for highly readable code with the engineering necessity of strict architectural containment. It uses a hybrid syntax that reads smoothly like an English instruction manual, but utilizes rigid [ ] brackets to guarantee visual structure and editor compatibility.
🌐 The Standalone IDE
You do not need to install anything to use Omnia.
Omnia is fully decentralized. The compiler engine runs natively in our web infrastructure. You can write, compile, and execute code directly in your browser using the official Web IDE:
Enter the Web Hub: omnia-lang.com
🚀 The Hybrid Syntax
sequence genesis [
    display "Signal received."
    
    set base_power to 1000
    set multiplier to 2
    
    set max_power to base_power * multiplier
    set final_output to max_power - 100
    
    display "Maximum power locked at:"
    display final_output
]

The Reference
 * sequence: Starts an orderly chain of logic (replaces function).
 * [ ]: Provides rigid containment fields for sequences. Essential for structural parsing.
 * display: Broadcasts data to the standard output.
 * set _ to _: Sets data into an indestructible memory lock. Once bound, variables in Omnia cannot be mutated.
 * Math Operators: Omnia fully supports +, -, *, and / for mathematical fusion.
⚙️ For Desktop Contributors (Rust)
While the Web IDE is the primary interface, the underlying engine is written in pure, zero-dependency Rust for operating system deployments.
 * Run locally: cargo run
 * Build binary: cargo build --release
🛡️ SLSA Level 3 Infrastructure
Every Rust binary created by this repository is cryptographically signed and hashed via Google's SLSA framework, ensuring absolute supply-chain security.

