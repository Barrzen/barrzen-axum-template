// Post-generation message and cleanup.
let name = variable::get("project-name");

if file::exists("README.project.md") {
  file::rename("README.project.md", "README.md");
}

if file::exists("AGENTS.project.md") {
  file::rename("AGENTS.project.md", "AGENTS.md");
}

if file::exists("hooks") {
  file::delete("hooks");
}

print("✅ Project generated: " + name);
print("\nNext steps:");
print("  cd " + name);
print("  # review .env if needed");
print("  cargo run\n");
print("Docs:");
print("  https://docs.rs/barrzen-axum-core");
print("  https://docs.rs/barrzen-axum-infra");
print("  https://docs.rs/barrzen-axum-obs");
print("  https://docs.rs/barrzen-axum-openapi");
print("  https://github.com/Barrzen/barrzen-axum-kit");
