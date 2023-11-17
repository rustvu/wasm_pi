import init from "./pkg/wasm_pi.js";

// Attach our wasm_pi function to the window object 
window.compute_pi = async () => {
    // Disable compute button while running
    document.getElementById("compute-button").disabled = true;

    // Get the parameter value from the input field
    const n_samples = parseInt(document.getElementById("n_samples").value);
    if (isNaN(n_samples)) {
        alert("Please enter a number");
    }
    else {
        // Instantiate our wasm module (or use the one that was already instantiated)
        const wasm_pi = await init();

        // Compute the result with WebAssembly
        try {
            const pi_estimate = wasm_pi.monte_carlo_pi(n_samples);
            
            // Set the result on the page
            document.getElementById("pi_estimate").textContent = pi_estimate;
        } catch (e) {
            alert("Wasm error: " + e.message);
        }
    }

    // Re-enable the compute button
    document.getElementById("compute-button").disabled = false;
};
