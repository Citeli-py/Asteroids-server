export function showScreen(name) {
  document.querySelectorAll(".screen").forEach((s) => s.classList.remove("active"));
  const overlay = document.getElementById("overlay");
  if (name) {
    overlay.classList.remove("hidden");
    document.getElementById(`screen-${name}`).classList.add("active");
  } else {
    overlay.classList.add("hidden");
  }
}
