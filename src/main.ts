import { invoke } from "@tauri-apps/api/tauri";

document.addEventListener("DOMContentLoaded", () => {
  // Fetch image bytes from the backend
  invoke<Uint8Array>("get_image")
    .then((response) => {
      // Create a Blob from the byte array
      const blob = new Blob([new Uint8Array(response)], { type: "image/png" });
      const imageUrl = URL.createObjectURL(blob);

      const image = document.createElement("img");
      const imageParent = document.getElementById("image-parent");

      if (imageParent) {
        const imageChild = imageParent.appendChild(image);
        imageChild.style.maxWidth = "100%";
        imageChild.src = imageUrl;

        // Notify backend that frontend is ready
        imageChild.addEventListener("load", () => {
          invoke("frontend_ready");
        });
      }
    })
    .catch((error) => {
      console.error("Error fetching image:", error);
    });
});
