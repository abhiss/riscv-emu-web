import init, { WasmLibRiscv } from "./riscv_emu_rust_wasm.js";

(async function () {
    console.log('Initializing.');
    await init();

    console.log('Executing instruction.');
    //addi a0,a0,12 -> 0b110001010000010100010011
    //slli a1,a1,0x3 -> 0x058e 


    document.getElementById('submit').onclick = function () {

        let results = document.getElementById('results');
        let value = parseInt(document.getElementById('encoded-input').value,2);

        //remove all registers
        while (results.firstChild) {
            results.removeChild(results.firstChild);
        }


        try {
            let json = (WasmLibRiscv.ExecuteInstruction(value));

            let registerArray = JSON.parse(json);
            for (let reg in registerArray) {
                let node = document.createElement('div');
                node.innerText = `Register: ${reg.padStart(2, 0)}   ----    Value: ${registerArray[reg]}`;
                results.append(node);
            }
        }
        catch (ex) {
            let node = document.createElement('div');
            node.innerText = `Invalid input`;
            results.append(node);
        }


    }

})();