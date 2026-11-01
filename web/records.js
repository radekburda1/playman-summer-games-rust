// Keep the existing five-line record format, scoped to this browser origin.
miniquad_add_plugin({
    name: "playman_records", version: 1,
    register_plugin(imports) {
        const key = "playman-records-v1";
        imports.env.playman_record_load = (slot) => {
            try {
                const line = (localStorage.getItem(key) || "").split("\n")[slot];
                if (!/^[0-9]+$/.test(line || "")) return -1;
                const value = Number(line);
                return Number.isInteger(value) && value <= 2147483647 ? value : -1;
            } catch (_) { return -1; }
        };
        imports.env.playman_record_save = (ptr, len) => {
            try {
                const text = new TextDecoder().decode(new Uint8Array(wasm_memory.buffer, ptr, len));
                localStorage.setItem(key, text);
                return localStorage.getItem(key) === text ? 1 : 0;
            } catch (_) { return 0; }
        };
    }
});
