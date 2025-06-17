pub fn generate_bunny_table() -> String {
    let mut rows = String::new();
    for bunny_fn in crate::COMMANDS {
        let cmd = bunny_fn();
        rows.push_str(&format!(
            r#"
            <tr class="border-b">
                <td class="px-4 py-2 text-left font-medium">{}</td>
                <td class="px-4 py-2 text-gray-700">{}</td>
            </tr>
            "#,
            cmd.name(),
            cmd.aliases().join(",")
        ));
    }

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Command Table</title>
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link href="https://cdn.jsdelivr.net/npm/tailwindcss@3.4.1/dist/tailwind.min.css" rel="stylesheet">
</head>
<body class="bg-gray-100 text-gray-900 font-sans">
    <div class="max-w-3xl mx-auto mt-10 p-6 bg-white shadow-lg rounded-lg">
        <h1 class="text-3xl font-bold mb-6 text-center">🐰 Bunny Index</h1>
        <table class="w-full table-auto border-collapse">
            <thead>
                <tr class="bg-gray-200">
                    <th class="px-4 py-2 text-left">Command</th>
                    <th class="px-4 py-2 text-left">Aliases</th>
                </tr>
            </thead>
            <tbody>
                {}
            </tbody>
        </table>
    </div>
</body>
</html>"#,
        rows
    )
}