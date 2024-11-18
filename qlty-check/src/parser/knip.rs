use super::Parser;
use anyhow::Result;
use qlty_types::analysis::v1::{Category, Issue, Level, Location};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
struct Report {
    // Unused files
    pub files: Vec<String>,

    // Issues within files
    pub issues: Vec<KnipIssue>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
struct KnipIssue {
    pub file: String,

    // Unused dependencies
    pub dependencies: Vec<KnipIdentifier>,
    #[serde(alias = "devDependencies")]
    pub dev_dependencies: Vec<KnipIdentifier>,
    #[serde(alias = "optionalPeerDependencies")]
    pub optional_peer_dependencies: Vec<KnipIdentifier>,

    // Unlisted dependencies
    pub unlisted: Vec<KnipIdentifier>,

    // Unlisted binaries
    pub binaries: Vec<KnipIdentifier>,

    // Unused exports
    pub exports: Vec<KnipIdentifier>,
    #[serde(alias = "nsExports")]
    pub ns_exports: Option<Vec<KnipIdentifier>>,

    // Unused exported types
    pub types: Vec<KnipIdentifier>,
    #[serde(alias = "nsTypes")]
    pub ns_types: Option<Vec<KnipIdentifier>>,

    // Unresolved imports
    pub unresolved: Vec<KnipIdentifier>,

    // Unused exported enum or class members
    #[serde(alias = "enumMembers")]
    pub enum_members: Option<HashMap<String, Vec<KnipIdentifier>>>,
    #[serde(alias = "classMembers")]
    pub class_members: Option<HashMap<String, Vec<KnipIdentifier>>>,

    // Duplicate exports
    pub duplicates: Vec<Vec<KnipIdentifier>>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
struct KnipIdentifier {
    pub name: String,
    pub line: Option<i32>,
    pub col: Option<i32>,
    pub pos: Option<i32>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Knip {}

impl Parser for Knip {
    fn parse(&self, _plugin_name: &str, output: &str) -> Result<Vec<Issue>> {
        let mut issues = vec![];
        let report: Report = serde_json::from_str(output)?;

        for file in report.files {
            issues.push(build_issue("unused-file", "Unused file".into(), file));
        }

        for issue in report.issues {
            for dependency in issue.dependencies {
                issues.push(build_issue(
                    "unused-dependency",
                    format!("Unused dependency: {}", dependency.name),
                    issue.file.clone(),
                ));
            }

            for dev_dependency in issue.dev_dependencies {
                issues.push(build_issue(
                    "unused-dev-dependency",
                    format!("Unused dev dependency: {}", dev_dependency.name),
                    issue.file.clone(),
                ));
            }

            for optional_peer_dependency in issue.optional_peer_dependencies {
                issues.push(build_issue(
                    "referenced-optional-peer-dependency",
                    format!(
                        "Referenced optional peer dependency: {}",
                        optional_peer_dependency.name
                    ),
                    issue.file.clone(),
                ));
            }

            for unlisted in issue.unlisted {
                issues.push(build_issue(
                    "unlisted-dependency",
                    format!("Unlisted dependency: {}", unlisted.name),
                    issue.file.clone(),
                ));
            }

            for binary in issue.binaries {
                issues.push(build_issue(
                    "unlisted-binary",
                    format!("Unlisted binary: {}", binary.name),
                    issue.file.clone(),
                ));
            }

            for unresolved in issue.unresolved {
                issues.push(build_issue(
                    "unresolved-import",
                    format!("Unresolved import: {}", unresolved.name),
                    issue.file.clone(),
                ));
            }

            for export in issue.exports {
                issues.push(build_issue(
                    "unused-export",
                    format!("Unused export: {}", export.name),
                    issue.file.clone(),
                ));
            }

            for ns_export in issue.ns_exports.unwrap_or_default() {
                issues.push(build_issue(
                    "export-in-used-namespace",
                    format!("Export in used namespace: {}", ns_export.name),
                    issue.file.clone(),
                ));
            }

            for export_type in issue.types {
                issues.push(build_issue(
                    "unused-exported-type",
                    format!("Unused exported type: {}", export_type.name),
                    issue.file.clone(),
                ));
            }

            for ns_type in issue.ns_types.unwrap_or_default() {
                issues.push(build_issue(
                    "exported-type-in-used-namespace",
                    format!("Exported type in used namespace: {}", ns_type.name),
                    issue.file.clone(),
                ));
            }

            for (enum_name, members) in issue.enum_members.unwrap_or_default() {
                for member in members {
                    issues.push(build_issue(
                        "unused-exported-enum-member",
                        format!("Unused exported enum member: {}.{}", enum_name, member.name),
                        issue.file.clone(),
                    ));
                }
            }

            for (class_name, members) in issue.class_members.unwrap_or_default() {
                for member in members {
                    issues.push(build_issue(
                        "unused-exported-class-member",
                        format!(
                            "Unused exported class member: {}.{}",
                            class_name, member.name
                        ),
                        issue.file.clone(),
                    ));
                }
            }

            for duplicate in issue.duplicates {
                let mut message = "Duplicate export: ".to_string();
                for instance in duplicate {
                    message.push_str(format!("Exporting '{}'", instance.name).as_str());

                    if let Some(line) = instance.line {
                        message.push_str(format!(" line: {}", line).as_str());
                    }

                    if let Some(col) = instance.col {
                        message.push_str(format!(" col: {}", col).as_str());
                    }

                    message.push_str(". ");
                }
                issues.push(build_issue(
                    "duplicate-export",
                    message.trim().into(),
                    issue.file.clone(),
                ));
            }
        }

        Ok(issues)
    }
}

fn build_issue(rule_key: &str, message: String, path: String) -> Issue {
    Issue {
        tool: "knip".into(),
        message,
        category: Category::DeadCode.into(),
        level: Level::Medium.into(),
        rule_key: rule_key.into(),
        location: Some(Location {
            path,
            ..Default::default()
        }),
        ..Default::default()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let input = r###"
        {
            "files": ["src/hello.js"],
            "issues": [
                {
                "file": "package.json",
                "dependencies": [
                    { "name": "@aws-sdk/client-batch" },
                    { "name": "@aws-sdk/client-cloudwatch-logs" }
                ],
                "devDependencies": [],
                "optionalPeerDependencies": [],
                "unlisted": [],
                "binaries": [],
                "unresolved": [],
                "exports": [],
                "types": [],
                "enumMembers": {},
                "duplicates": []
                },
                {
                "file": "index.js",
                "dependencies": [],
                "devDependencies": [],
                "optionalPeerDependencies": [],
                "unlisted": [{ "name": "src/mistake.js" }],
                "binaries": [],
                "unresolved": [],
                "exports": [],
                "types": [],
                "enumMembers": {},
                "duplicates": []
                },
                {
                "file": "refresher.js",
                "dependencies": [],
                "devDependencies": [],
                "optionalPeerDependencies": [],
                "unlisted": [],
                "binaries": [],
                "unresolved": [],
                "exports": [],
                "types": [],
                "enumMembers": {},
                "duplicates":[[{"name":"Refresher","line":29,"col":5,"pos":835},{"name":"default","line":223,"col":15,"pos":7052}]]
                }
            ]
        }
        "###;

        let issues = Knip::default().parse("knip", input);
        insta::assert_yaml_snapshot!(issues.unwrap(), @r#"
        - tool: knip
          ruleKey: unused-file
          message: Unused file
          level: LEVEL_MEDIUM
          category: CATEGORY_DEAD_CODE
          location:
            path: src/hello.js
        - tool: knip
          ruleKey: unused-dependency
          message: "Unused dependency: @aws-sdk/client-batch"
          level: LEVEL_MEDIUM
          category: CATEGORY_DEAD_CODE
          location:
            path: package.json
        - tool: knip
          ruleKey: unused-dependency
          message: "Unused dependency: @aws-sdk/client-cloudwatch-logs"
          level: LEVEL_MEDIUM
          category: CATEGORY_DEAD_CODE
          location:
            path: package.json
        - tool: knip
          ruleKey: unlisted-dependency
          message: "Unlisted dependency: src/mistake.js"
          level: LEVEL_MEDIUM
          category: CATEGORY_DEAD_CODE
          location:
            path: index.js
        - tool: knip
          ruleKey: duplicate-export
          message: "Duplicate export: Exporting 'Refresher' line: 29 col: 5. Exporting 'default' line: 223 col: 15."
          level: LEVEL_MEDIUM
          category: CATEGORY_DEAD_CODE
          location:
            path: refresher.js
        "#);
    }
}
