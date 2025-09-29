import { invoke } from "@tauri-apps/api/core";

let input: HTMLInputElement | null;

async function updateUrl() {
  if (input) {
    console.log("Input value:", input.value);
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    input.textContent = await invoke("save_url", {
      url: input.value,
    });
  }
}

window.addEventListener("DOMContentLoaded", () => {
  input = document.querySelector("#url-input")
  document.querySelector("#save-form")?.addEventListener("submit", e => {
    e.preventDefault();
    updateUrl();
  });
});
