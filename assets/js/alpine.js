if (window.Alpine) {
  console.log("Alpine was already initialized");
} else {
  document.addEventListener("alpine:init", () => {
    console.log("Alpine loaded");
    Alpine.data("todoItem", () => ({
      editing: false,

      startEdit() {
        this.editing = true;
        this.$nextTick(() => this.$refs.editInput.focus());
      },

      cancelEdit() {
        this.editing = false;
      },
    }));
  });
}
