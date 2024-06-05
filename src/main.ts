import { invoke } from "@tauri-apps/api/tauri";

// Generate image on startup to avoid blank image icon
// ! will need to think of a better way for this lmao
fetch("http://127.0.0.1:4309/image")
  .then((response) => {
    if (!response.ok) {
      throw new Error("Network response was not ok");
    }
    return response.blob();
  })
  .then((blob) => {
    const imageUrl = URL.createObjectURL(blob);
    const image = document.createElement("img");
    // @ts-ignore
    const image_child = document
      .getElementById("image-parent")
      .appendChild(image);
    image_child.style.maxWidth = "100%";
    image_child.src = imageUrl;

    // Wait for image to fully load
    image_child.addEventListener("load", () => {
      // Tell the backend the DOM fully loaded
      if (typeof window !== "undefined") {
        // * dev testing
        /* const tempTxtElement = document.getElementById("temp-txt");
        if (tempTxtElement) {
          tempTxtElement.innerText = "Loaded!";
        } */
        invoke("frontend_ready");
      }
    });
  })
  .catch((error) => {
    console.error("There has been a problem with your fetch operation:", error);
  });
