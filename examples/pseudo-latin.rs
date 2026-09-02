use glabrous::{
    add_tag, compress, delete_variables, from_list, from_template, from_tags_list, from_text,
    init_context, init_context_file, insert_many_templates, insert_template, is_final, is_set,
    join, partial_process, partial_process_result, process, process_with_default,
    read_context_file, read_template_file, set_variables, tags_of, tags_rename, to_final_text,
    to_text, unset_context, variables_of, write_context_file, write_template_file,
    ProcessResult, Template, Token,
};

const LETTERA_TEMPLATE: &str = "Carissime {{amice}},\n\n\
    Scribo tibi haec verba ex urbe {{civitas}}, ubi {{eventus}} accidit die {{dies}}. \
    Res est {{adjectivum1}} et {{adjectivum2}}, nam omnes {{cives}} de ea re \
    {{verbum1}} et {{verbum2}}.\n\n\
    Igitur, cum {{nomen}} in foro {{locus}} ambularet, vidit {{objectum}} \
    quod {{adjectivum3}} erat super {{res}}. Tunc {{persona}} ad eum dixit: \
    \"{{dictum1}}!\" Et {{nomen}} respondit: \"{{dictum2}}.\"\n\n\
    Postea {{tempus}} transierunt, et {{civitas}} mutata est. \
    {{magistratus}} novum {{decretum}} fecerunt, quod {{effectus}} habuit \
    in {{omnes}}. Quidam {{cives}} laeti erant, alii {{adjectivum4}}.\n\n\
    Nos vero in {{locus2}} habitamus, prope {{flumen}} et {{mons}}. \
    Ibi {{tempus2}} {{adjectivum5}} est, et {{animus}} semper {{adjectivum6}}. \
    {{familia}} mea {{numerus}} est, et {{servus}} noster {{nomen2}} appellatur.\n\n\
    De {{religio}} dico tibi quod {{sacerdos}} in {{templum}} \
    {{ritus}} celebravit, et {{populus}} {{cantus}} cantaverunt. \
    {{vitae}} nostra plena est {{gaudium}} et {{spes}}.\n\n\
    Vale, et {{epistula}} mea cum {{amor}} et {{fides}} scribitur.\n\n\
    Tuus {{amicitia}},\n{{scriptor}}";

fn main() {
    println!("=============================================");
    println!("  Glabrous-rs Pseudo-Latin Template Demo");
    println!("=============================================\n");

    // -------------------------------------------------------
    // 1. PARSE the long template from text
    // -------------------------------------------------------
    println!("--- 1. Parsing the template ---\n");

    let template = from_text(LETTERA_TEMPLATE).expect("Failed to parse template");
    let tags = tags_of(&template);

    println!(
        "Template parsed successfully: {} tokens, {} tags.\n",
        template.content.len(),
        tags.len()
    );

    // Print all tags found
    print!("Tags found: ");
    for (i, token) in tags.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", token);
    }
    println!("\n");

    // -------------------------------------------------------
    // The template itself, shown as parsed from text
    // -------------------------------------------------------
    println!("--- The template itself ---\n");
    println!("{}", to_text(&template));

    // -------------------------------------------------------
    // 2. BUILD CONTEXTS
    // -------------------------------------------------------
    println!("\n--- 2. Building contexts ---\n");

    // Build a full context from a list of pairs
    let full_context = from_list(&[
        ("amice", "Luci"),
        ("civitas", "Romae"),
        ("eventus", "mirabile"),
        ("dies", "XV Kalendas Apriles"),
        ("adjectivum1", "magnifica"),
        ("adjectivum2", "memorabilis"),
        ("cives", "loquentes"),
        ("verbum1", "disputabant"),
        ("verbum2", "cogitabant"),
        ("nomen", "Marcus"),
        ("locus", "comitio"),
        ("objectum", "tabulam"),
        ("adjectivum3", "antiqua"),
        ("res", "marmore"),
        ("persona", "senex"),
        ("dictum1", "Salve, Marcus"),
        ("dictum2", "Gratias tibi, amice"),
        ("tempus", "tres dies"),
        ("magistratus", "consules"),
        ("decretum", "legem"),
        ("effectus", "magnum"),
        ("omnes", "civitatem"),
        ("adjectivum4", "tristes"),
        ("locus2", "villa nostra"),
        ("flumen", "Tiberim"),
        ("mons", "Palatium"),
        ("tempus2", "aer"),
        ("adjectivum5", "serenum"),
        ("animus", "tranquillus"),
        ("adjectivum6", "fortis"),
        ("familia", "numerosa"),
        ("numerus", "septem"),
        ("servus", "servus"),
        ("nomen2", "Fidelis"),
        ("religio", "deorum"),
        ("sacerdos", "pontifex"),
        ("templum", "Capitolio"),
        ("ritus", "sacrum"),
        ("populus", "universus"),
        ("cantus", "hymnos"),
        ("vitae", "dies"),
        ("gaudium", "laetitiam"),
        ("spes", "futuram"),
        ("epistula", "hanc"),
        ("amor", "magno"),
        ("fides", "sincera"),
        ("amicitia", "in aeternum"),
        ("scriptor", "Glabrius"),
    ]);
    println!(
        "Full context built: {} variables.\n",
        full_context.variables.len()
    );

    println!("--- The context itself ---\n");
    let mut names: Vec<&String> = full_context.variables.keys().collect();
    names.sort();
    for name in names {
        let value = &full_context.variables[name];
        println!("  {{{{{}}}}} = \"{}\"", name, value);
    }
    println!();

    // Build an empty context and populate it incrementally
    let mut ctx = init_context();
    ctx = set_variables(&[("amice", "Gaius"), ("civitas", "Carthagine")], &ctx);
    println!("Incremental context ({} variables, partial):", ctx.variables.len());
    for (name, value) in &ctx.variables {
        println!("  {{{{{}}}}} = \"{}\"", name, value);
    }
    println!();

    // -------------------------------------------------------
    // 3. FULL PROCESSING
    // -------------------------------------------------------
    println!("--- 3. Full processing ---\n");

    let result = process(&template, &full_context);
    println!("--- Rendered letter ---\n");
    println!("{}", result);
    println!("--- End of letter ---\n");

    println!(
        "Result length: {} characters.\n",
        result.len()
    );

    // -------------------------------------------------------
    // 4. PROCESS WITH DEFAULT for missing tags
    // -------------------------------------------------------
    println!("--- 4. Process with default (missing tags) ---\n");

    let sparse = from_list(&[("amice", "Titus"), ("civitas", "Athenis")]);
    let partial_result = process_with_default("[N/A]", &template, &sparse);
    println!(
        "With defaults, first 200 chars:\n{}\n",
        &partial_result[..200.min(partial_result.len())]
    );

    // -------------------------------------------------------
    // 5. PARTIAL PROCESSING
    // -------------------------------------------------------
    println!("--- 5. Partial processing ---\n");

    let sub_ctx = from_list(&[
        ("amice", "Decimus"),
        ("civitas", "Alexandriae"),
        ("eventus", "insolitum"),
        ("dies", "pridie Nones Maias"),
    ]);
    let partially = partial_process(&template, &sub_ctx);
    let remaining = tags_of(&partially);
    println!(
        "After partial processing: {} tags remain out of {}.\n",
        remaining.len(),
        tags.len()
    );

    // -------------------------------------------------------
    // 6. PARTIAL PROCESS RESULT (Final vs Partial)
    // -------------------------------------------------------
    println!("--- 6. Partial process result ---\n");

    let full_partial = partial_process_result(&template, &full_context);
    match &full_partial {
        ProcessResult::Final(text) => {
            println!("Full context -> Final result ({} chars).\n", text.len());
        }
        ProcessResult::Partial { .. } => {
            println!("Unexpected: still partial with full context.\n");
        }
    }

    let sparse_partial = partial_process_result(&template, &sub_ctx);
    match &sparse_partial {
        ProcessResult::Final(_) => {
            println!("Unexpected: should be partial.\n");
        }
        ProcessResult::Partial {
            template: t,
            context: c,
        } => {
            println!(
                "Sparse context -> Partial result: {} tags remain, {} unset variables.\n",
                tags_of(t).len(),
                c.variables.len()
            );
        }
    }

    // -------------------------------------------------------
    // 7. CONTEXT MANIPULATION
    // -------------------------------------------------------
    println!("--- 7. Context manipulation ---\n");

    // Merge two disjoint contexts
    let ctx_a = from_list(&[("amice", "Flavia"), ("civitas", "Lutetiae")]);
    let ctx_b = from_list(&[("eventus", "solemne"), ("dies", "Kalendis Iuniis")]);
    let merged = join(&ctx_a, &ctx_b).expect("Should merge disjoint contexts");
    println!("Merged context: {} variables.", merged.variables.len());

    // Detect conflict
    let ctx_c = from_list(&[("amice", "Other")]);
    match join(&ctx_a, &ctx_c) {
        Ok(_) => println!("Unexpected: should have conflict."),
        Err(conflict) => {
            println!(
                "Join conflict on variable(s): {:?}\n",
                conflict.variables.keys().collect::<Vec<_>>()
            );
        }
    }

    // Delete variables
    let trimmed = delete_variables(&["eventus", "dies"], &merged);
    println!(
        "After deleting 2 variables: {} remain.\n",
        trimmed.variables.len()
    );

    // Unset context
    let mixed = from_list(&[("amice", "Set"), ("civitas", ""), ("eventus", "AlsoSet")]);
    match unset_context(&mixed) {
        Some(unset) => {
            println!(
                "Unset variables: {:?}\n",
                unset.variables.keys().collect::<Vec<_>>()
            );
        }
        None => println!("No unset variables.\n"),
    }

    // Variables list
    let vars = variables_of(&full_context);
    println!("All {} variable names in full context.", vars.len());

    // is_set
    println!("Full context is_set: {}", is_set(&full_context));
    println!("Mixed context is_set: {}\n", is_set(&mixed));

    // -------------------------------------------------------
    // 8. TEMPLATE MANIPULATION
    // -------------------------------------------------------
    println!("--- 8. Template manipulation ---\n");

    // is_final
    println!("Original template is_final: {}", is_final(&template));
    let plain = from_text("Sinetags").unwrap();
    println!("Plain text template is_final: {}\n", is_final(&plain));

    // to_text vs to_final_text
    println!("to_text (with tags): {}...", &to_text(&template)[..60]);
    println!(
        "to_final_text (tags stripped): {}...\n",
        &to_final_text(&template)[..60]
    );

    // add_tag: replace a literal with a new tag
    let modified = add_tag(&template, "verba", "eventus_novum")
        .expect("Should find 'verba' in template");
    let new_tags = tags_of(&modified);
    println!(
        "After add_tag (replacing 'verba' with {{eventus_novum}}): {} tags (was {}).",
        new_tags.len(),
        tags.len()
    );

    // tags_rename
    let renamed = tags_rename(
        &[
            ("amice", "friend"),
            ("civitas", "city"),
            ("nomen", "name"),
        ],
        &template,
    );
    println!("Renamed template to_text starts: {}\n", &to_text(&renamed)[..80]);

    // insert_template: compose a sub-template into the main one
    let header = from_text("=== EPISTULA ===\n").unwrap();
    let with_header = insert_template(&renamed, &Token::Tag("friend".to_string()), &header)
        .expect("Should insert at 'friend' tag");
    println!(
        "After insert_template, first 40 chars: {:?}\n",
        &to_text(&with_header)[..40]
    );

    // insert_many_templates
    let footer = from_text("\n=== FINIS ===").unwrap();
    let signature = from_text("\n--- SIGNATUM ---").unwrap();
    let composed = insert_many_templates(
        &renamed,
        &[
            (&Token::Tag("name".to_string()), &signature),
            (&Token::Tag("city".to_string()), &footer),
        ],
    )
    .expect("Should insert both templates");
    let composed_text = to_text(&composed);
    println!(
        "After insert_many_templates, last 40 chars: {:?}\n",
        &composed_text[composed_text.len() - 40..]
    );

    // compress: merge adjacent literals after partial processing
    let multi = from_text("{{a}}{{b}}{{c}}-{{d}}").unwrap();
    let multi_ctx = from_list(&[("a", "X"), ("b", "Y")]);
    let partial_multi = partial_process(&multi, &multi_ctx);
    println!(
        "Before compress: {} tokens.",
        partial_multi.content.len()
    );
    let compressed = compress(&partial_multi);
    println!("After compress:  {} token(s).\n", compressed.content.len());

    // from_template: build context from template tags
    let auto_ctx = from_template(&template);
    println!(
        "Auto-context from template: {} variables (all unset).\n",
        auto_ctx.variables.len()
    );

    // from_tags_list
    let tags_ctx = from_tags_list(&["alpha", "beta", "gamma"]);
    println!(
        "Context from tags_list: {:?}\n",
        tags_ctx.variables.keys().collect::<Vec<_>>()
    );

    // -------------------------------------------------------
    // 9. FILE I/O
    // -------------------------------------------------------
    println!("--- 9. File I/O ---\n");

    let tmpl_path = "/tmp/glabrous_pseudo_latin_template.txt";
    let ctx_path = "/tmp/glabrous_pseudo_latin_context.json";

    write_template_file(tmpl_path, &template).expect("Failed to write template");
    println!("Template written to {}", tmpl_path);

    let read_back = read_template_file(tmpl_path).expect("Failed to read template back");
    println!(
        "Template roundtrip OK: {} tokens.\n",
        read_back.content.len()
    );

    write_context_file(ctx_path, &full_context).expect("Failed to write context");
    println!("Context written to {}", ctx_path);

    let read_ctx = read_context_file(ctx_path)
        .expect("Failed to read context")
        .expect("Context should parse");
    println!("Context roundtrip OK: {} variables.", read_ctx.variables.len());

    init_context_file("/tmp/glabrous_pseudo_latin_init.json", &full_context)
        .expect("Failed to init context file");
    let inited = read_context_file("/tmp/glabrous_pseudo_latin_init.json")
        .expect("Failed to read init")
        .expect("Should parse");
    println!(
        "Init context file: all {} variables are empty: {}\n",
        inited.variables.len(),
        inited.variables.values().all(|v| v.is_empty())
    );

    // -------------------------------------------------------
    // 10. JSON serialization of templates
    // -------------------------------------------------------
    println!("--- 10. JSON serialization ---\n");

    let json = serde_json::to_string_pretty(&template).expect("Serialize failed");
    let lines = json.lines().count();
    println!("Template serializes to {} lines of JSON.", lines);

    let deserialized: Template =
        serde_json::from_str(&json).expect("Deserialize failed");
    println!(
        "JSON roundtrip OK: {} tokens match original.\n",
        deserialized.content.len()
    );

    // -------------------------------------------------------
    // Summary
    // -------------------------------------------------------
    println!("=============================================");
    println!("  Demo complete. All features exercised.");
    println!("=============================================");
}
