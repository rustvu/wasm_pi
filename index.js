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
        // Instantiate our wasm module
        const wasm_pi = await init();

        // Call the Add function export from wasm, save the result
        const addResult = wasm_pi.monte_carlo_pi(n_samples);

        // Set the result on the page
        document.getElementById("result").textContent = addResult;
    }

    // Re-enable the compute button
    document.getElementById("compute-button").disabled = false;
};
