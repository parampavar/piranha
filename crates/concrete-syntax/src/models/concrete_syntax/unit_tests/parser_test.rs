/*
 Copyright (c) 2023 Uber Technologies, Inc.

 <p>Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file
 except in compliance with the License. You may obtain a copy of the License at
 <p>http://www.apache.org/licenses/LICENSE-2.0

 <p>Unless required by applicable law or agreed to in writing, software distributed under the
 License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
 express or implied. See the License for the specific language governing permissions and
 limitations under the License.
*/

use super::super::parser::*;
use super::super::resolver::ResolvedCsElement;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_simple_capture() {
    let input = ":[var]";
    let result = ConcreteSyntax::parse(input).unwrap();
    let elements = result.pattern.sequence;

    assert_eq!(elements.len(), 1);
    match &elements[0] {
      CsElement::Capture { name, mode } => {
        assert_eq!(name, "var");
        assert_eq!(*mode, CaptureMode::Single);
      }
      _ => panic!("Expected capture, got: {:?}", elements[0]),
    }
  }

  #[test]
  fn test_parse_literal() {
    let input = "public int";
    let result = ConcreteSyntax::parse(input).unwrap();
    let elements = result.pattern.sequence;

    assert_eq!(elements.len(), 2);
    match &elements[0] {
      CsElement::Literal(text) => {
        assert_eq!(text, "public");
      }
      _ => panic!("Expected literal, got: {:?}", elements[0]),
    }
    match &elements[1] {
      CsElement::Literal(text) => {
        assert_eq!(text, "int");
      }
      _ => panic!("Expected literal, got: {:?}", elements[0]),
    }
  }

  #[test]
  fn test_parse_capture_with_literal() {
    let input = "function @name() {";
    let result = ConcreteSyntax::parse(input).unwrap();
    let elements = result.pattern.sequence;

    assert_eq!(elements.len(), 4);

    // Should be: literal "function ", capture "name", literal "() {"
    match &elements[0] {
      CsElement::Literal(text) => assert_eq!(text, "function"),
      _ => panic!("Expected literal, got: {:?}", elements[0]),
    }

    match &elements[1] {
      CsElement::Capture { name, mode } => {
        assert_eq!(name, "name");
        assert_eq!(*mode, CaptureMode::Single);
      }
      _ => panic!("Expected capture, got: {:?}", elements[1]),
    }

    match &elements[2] {
      CsElement::Literal(text) => assert_eq!(text, "()"),
      _ => panic!("Expected literal, got: {:?}", elements[2]),
    }

    match &elements[3] {
      CsElement::Literal(text) => assert_eq!(text, "{"),
      _ => panic!("Expected literal, got: {:?}", elements[3]),
    }
  }

  #[test]
  fn test_parse_capture_modes() {
    let input = ":[single] :[many+] :[optional*]";
    let result = ConcreteSyntax::parse(input).unwrap();
    let elements = result.pattern.sequence;

    // Check the captures specifically - the actual number may vary
    let captures: Vec<&CsElement> = elements
      .iter()
      .filter(|e| matches!(e, CsElement::Capture { .. }))
      .collect();

    assert_eq!(captures.len(), 3);

    match captures[0] {
      CsElement::Capture { name, mode } => {
        assert_eq!(name, "single");
        assert_eq!(*mode, CaptureMode::Single);
      }
      _ => panic!("Expected capture"),
    }

    match captures[1] {
      CsElement::Capture { name, mode } => {
        assert_eq!(name, "many");
        assert_eq!(*mode, CaptureMode::OnePlus);
      }
      _ => panic!("Expected capture"),
    }

    match captures[2] {
      CsElement::Capture { name, mode } => {
        assert_eq!(name, "optional");
        assert_eq!(*mode, CaptureMode::ZeroPlus);
      }
      _ => panic!("Expected capture"),
    }
  }

  #[test]
  fn test_parse_complex_pattern() {
    let input = "if (:[condition+]) { :[body*] }";
    let result = ConcreteSyntax::parse(input).unwrap();
    let elements = result.pattern.sequence;

    // Find the captures
    let captures: Vec<&CsElement> = elements
      .iter()
      .filter(|e| matches!(e, CsElement::Capture { .. }))
      .collect();

    assert_eq!(captures.len(), 2);

    // Check condition capture
    match captures[0] {
      CsElement::Capture { name, mode } => {
        assert_eq!(name, "condition");
        assert_eq!(*mode, CaptureMode::OnePlus);
      }
      _ => panic!("Expected condition capture"),
    }

    // Check body capture
    match captures[1] {
      CsElement::Capture { name, mode } => {
        assert_eq!(name, "body");
        assert_eq!(*mode, CaptureMode::ZeroPlus);
      }
      _ => panic!("Expected body capture"),
    }
  }

  #[test]
  fn test_parse_identifier_variations() {
    // Test different valid identifier patterns
    // Note: identifiers can start with ASCII_ALPHA or "_" per grammar
    let test_cases = vec![
      (":[a]", "a"),
      (":[variable_name]", "variable_name"),
      (":[var123]", "var123"),
      (":[CamelCase]", "CamelCase"),
      (":[_underscore]", "_underscore"),
      (":[name_with_underscore]", "name_with_underscore"),
    ];

    for (input, expected_name) in test_cases {
      let result = ConcreteSyntax::parse(input).unwrap();
      let elements = result.pattern.sequence;

      assert_eq!(elements.len(), 1);
      match &elements[0] {
        CsElement::Capture { name, mode } => {
          assert_eq!(name, expected_name);
          assert_eq!(*mode, CaptureMode::Single);
        }
        _ => panic!("Expected capture for input: {input}"),
      }
    }
  }

  #[test]
  fn test_parse_whitespace_handling() {
    let input = "  function   :[name]   ()  ";
    let result = ConcreteSyntax::parse(input).unwrap();
    let elements = result.pattern.sequence;

    // Whitespace should be preserved in literals but trimmed in the parsing
    assert!(elements.len() >= 3);

    // Find the capture
    let capture = elements
      .iter()
      .find(|e| matches!(e, CsElement::Capture { .. }))
      .expect("Should have a capture");

    match capture {
      CsElement::Capture { name, mode } => {
        assert_eq!(name, "name");
        assert_eq!(*mode, CaptureMode::Single);
      }
      _ => panic!("Expected capture"),
    }
  }

  #[test]
  fn test_parse_error_cases() {
    let error_cases = vec![
      ":[",          // Incomplete capture
      ":[123]",      // Invalid identifier (starts with number)
      ":[var++]",    // Invalid capture mode
      ":[]",         // Empty identifier
      ":[123var]",   // Invalid identifier (starts with number)
      ":[var-name]", // Invalid identifier (contains hyphen)
    ];

    for input in error_cases {
      let result = ConcreteSyntax::parse(input);
      assert!(result.is_err(), "Expected error for input: {input}");
    }
  }

  #[test]
  fn test_parse_empty_input() {
    let result = ConcreteSyntax::parse("");
    assert!(result.is_err(), "Empty input should produce an error");
  }

  #[test]
  fn test_parse_only_whitespace() {
    let result = ConcreteSyntax::parse("   ");
    // This might pass depending on grammar - whitespace handling
    // If it passes, it should create an empty sequence or fail gracefully
    match result {
      Ok(cs) => {
        let elements = cs.pattern.sequence;

        // Should be empty or contain only whitespace literals
        assert!(
          elements.is_empty()
            || elements
              .iter()
              .all(|e| matches!(e, CsElement::Literal(s) if s.trim().is_empty()))
        );
      }
      Err(_) => {
        // Also acceptable - depends on grammar requirements
      }
    }
  }

  #[test]
  fn test_debug_output() {
    let input = ":[var+]";
    let result = ConcreteSyntax::parse(input).unwrap();

    // Test that Debug trait works (useful for debugging)
    let debug_str = format!("{result:?}");
    assert!(debug_str.contains("var"));
    assert!(debug_str.contains("OnePlus"));
  }

  #[test]
  fn test_where() {
    let input = ":[var+] |> :[var] in [\"a\"]";
    let result = ConcreteSyntax::parse(input).unwrap();

    // Check the pattern elements
    let elements = &result.pattern.sequence;
    assert_eq!(elements.len(), 1);

    match &elements[0] {
      CsElement::Capture { name, mode } => {
        assert_eq!(name, "var");
        assert_eq!(*mode, CaptureMode::OnePlus);
      }
      _ => panic!("Expected capture"),
    }

    // Check the constraints
    let constraints = &result.pattern.constraints;
    assert_eq!(constraints.len(), 1);

    match &constraints[0] {
      CsConstraint::In { capture, items } => {
        assert_eq!(capture, "var");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0], "a");
      }
      _ => panic!("Expected In constraint"),
    }

    // Test that Debug trait works (useful for debugging)
    let debug_str = format!("{result:?}");
    assert!(debug_str.contains("var"));
    assert!(debug_str.contains("OnePlus"));
    assert!(debug_str.contains("constraints"));
  }

  #[test]
  fn test_parse_regex_constraint() {
    let input = ":[var] |> :[var] matches /^test.*/";
    let result = ConcreteSyntax::parse(input).unwrap();

    let constraints = &result.pattern.constraints;
    assert_eq!(constraints.len(), 1);

    match &constraints[0] {
      CsConstraint::Regex { capture, pattern } => {
        assert_eq!(capture, "var");
        assert_eq!(pattern, "^test.*");
      }
      _ => panic!("Expected regex constraint"),
    }
  }

  #[test]
  fn test_parse_regex_with_escaped_chars() {
    let input = ":[var] |> :[var] matches /te\\/st.*/";
    let result = ConcreteSyntax::parse(input).unwrap();

    let constraints = &result.pattern.constraints;
    match &constraints[0] {
      CsConstraint::Regex { pattern, .. } => {
        assert_eq!(pattern, "te/st.*"); // Should be unescaped
      }
      _ => panic!("Expected regex constraint"),
    }
  }

  #[test]
  fn test_parse_mixed_constraints() {
    let input = r#":[var] |> :[var] in ["a"], :[var] matches /^test.*/"#;
    let result = ConcreteSyntax::parse(input).unwrap();

    let constraints = &result.pattern.constraints;
    assert_eq!(constraints.len(), 2);
    assert!(matches!(constraints[0], CsConstraint::In { .. }));
    assert!(matches!(constraints[1], CsConstraint::Regex { .. }));
  }

  #[test]
  fn test_parse_and_resolve_flow() {
    let input = ":[var+] |> :[var] in [\"a\"]";

    // Parse
    let parsed = ConcreteSyntax::parse(input).unwrap();

    // Verify parsing separated constraints from elements
    assert_eq!(parsed.pattern.sequence.len(), 1);
    assert_eq!(parsed.pattern.constraints.len(), 1);

    // Resolve
    let resolved = parsed.resolve().unwrap();

    // Verify resolution attached constraints to captures
    assert_eq!(resolved.pattern.sequence.len(), 1);

    match &resolved.pattern.sequence[0] {
      ResolvedCsElement::Capture {
        name,
        mode,
        constraints,
      } => {
        assert_eq!(name, "var");
        assert_eq!(*mode, CaptureMode::OnePlus);
        assert_eq!(constraints.len(), 1);

        match &constraints[0] {
          CsConstraint::In { capture, items } => {
            assert_eq!(capture, "var");
            assert_eq!(items.len(), 1);
            assert_eq!(items[0], "a");
          }
          _ => panic!("Expected In constraint"),
        }
      }
      _ => panic!("Expected capture with constraints"),
    }
  }

  #[test]
  fn test_parse_negated_constraints() {
    // Test "not in"
    let result = ConcreteSyntax::parse(":[var] |> :[var] not in [\"a\"]").unwrap();
    assert!(matches!(
      result.pattern.constraints[0],
      CsConstraint::Not(_)
    ));

    // Test "not matches"
    let result = ConcreteSyntax::parse(":[var] |> :[var] not matches /test/").unwrap();
    assert!(matches!(
      result.pattern.constraints[0],
      CsConstraint::Not(_)
    ));
  }

  #[test]
  fn test_parse_contains_constraint() {
    let input = ":[x] |> :[x] contains /:[y]/";
    let result = ConcreteSyntax::parse(input).unwrap();

    // Check constraints
    let constraints = &result.pattern.constraints;
    assert_eq!(constraints.len(), 1);

    match &constraints[0] {
      CsConstraint::Contains {
        target, pattern, ..
      } => {
        assert_eq!(target, "x");
        assert_eq!(pattern.len(), 1); // just capture

        match &pattern[0] {
          CsElement::Capture { name, .. } => assert_eq!(name, "y"),
          _ => panic!("Expected capture"),
        }
      }
      _ => panic!("Expected Contains constraint"),
    }
  }

  #[test]
  fn test_parse_contains_constraint_with_literal() {
    let input = ":[x] |> :[x] contains /:[y].Return()/";
    let result = ConcreteSyntax::parse(input).unwrap();

    // Check constraints
    let constraints = &result.pattern.constraints;
    assert_eq!(constraints.len(), 1);

    match &constraints[0] {
      CsConstraint::Contains {
        target, pattern, ..
      } => {
        assert_eq!(target, "x");
        assert_eq!(pattern.len(), 2); // capture + literal

        match &pattern[0] {
          CsElement::Capture { name, .. } => assert_eq!(name, "y"),
          _ => panic!("Expected capture"),
        }

        match &pattern[1] {
          CsElement::Literal(text) => assert_eq!(text, ".Return()"),
          _ => panic!("Expected literal"),
        }
      }
      _ => panic!("Expected Contains constraint"),
    }
  }

  #[test]
  fn test_parse_contains_constraint_with_escaped_slashes() {
    let input = ":[x] |> :[x] contains /path\\/to\\/file/";
    let result = ConcreteSyntax::parse(input).unwrap();

    let constraints = &result.pattern.constraints;
    assert_eq!(constraints.len(), 1);

    match &constraints[0] {
      CsConstraint::Contains {
        target, pattern, ..
      } => {
        assert_eq!(target, "x");
        assert_eq!(pattern.len(), 1);

        // The escaped slashes should be unescaped in the pattern
        match &pattern[0] {
          CsElement::Literal(text) => {
            assert_eq!(text, "path/to/file"); // Should be unescaped
          }
          _ => panic!("Expected literal element with unescaped text"),
        }
      }
      _ => panic!("Expected Contains constraint"),
    }
  }

  #[test]
  fn test_parse_contains_constraint_with_root_fails() {
    // This should fail during resolution, not parsing
    let input = ":[x] |> root contains /:[y]/";
    let parse_result = ConcreteSyntax::parse(input);
    assert!(parse_result.is_ok()); // Parsing should succeed

    // But resolution should fail
    let resolved = parse_result.unwrap().resolve();
    assert!(resolved.is_err());
    assert!(resolved
      .unwrap_err()
      .contains("'root contains' is not supported"));
  }
}
