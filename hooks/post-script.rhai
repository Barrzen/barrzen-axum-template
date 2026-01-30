// Post-generation message and cleanup.
let name = variable::get("project-name");

if file::exists("README.project.md") {
  file::rename("README.project.md", "README.md");
}

if file::exists("hooks") {
  file::delete("hooks");
}

print("✅ Project generated: " + name);
print("\nNext steps:");
print("  cd " + name);
print("  cp .env.example .env");
print("  cargo run\n");
print("Docs:");
print("  https://docs.rs/barrzen-axum-core");
print("  https://docs.rs/barrzen-axum-infra");
print("  https://docs.rs/barrzen-axum-obs");
print("  https://docs.rs/barrzen-axum-openapi");
print("  https://github.com/Barrzen/barrzen-axum-kit");
