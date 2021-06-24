#!/usr/bin/bash
mkdir Semgrep\ Results
printf "\n1. XSS RULESETS\n\n"
semgrep --config "p/xss" > Semgrep\ Results/xss.txt ;
printf "\n2. JWT RULESETS\n\n"
semgrep --config "p/jwt" > Semgrep\ Results/jwt.txt ;
printf "\n3. INSECURE TRANSPORT RULESETS\n\n"
semgrep --config "p/insecure-transport" > Semgrep\ Results/insecure-transport.txt ;
printf "\n4. COMMAND INJECTION RULESETS\n\n"
semgrep --config "p/command-injection" > Semgrep\ Results/command-injection.txt ;
printf "\n5. R2C-CI RULESETS\n\n"
semgrep --config "p/r2c-ci" > Semgrep\ Results/r2c-ci.txt ;
printf "\n6. R2C-SECURITY-AUDIT RULESETS\n\n"
semgrep --config "p/r2c-security-audit" > Semgrep\ Results/r2c-security-audit.txt ;
printf "\n7. ECURITY-AUDIT RULESETS\n\n"
semgrep --config "p/security-audit" > Semgrep\ Results/security-audit.txt ;
printf "\n8. JAVA RULESETS\n\n"
semgrep --config "p/java" > Semgrep\ Results/java.txt ;
printf "\nS9. ECRETS RULESETS\n\n"
semgrep --config "p/secrets" > Semgrep\ Results/secrets.txt ;
printf "\n10. FINDSECBUGS RULESETS\n\n"
semgrep --config "p/findsecbugs" > Semgrep\ Results/findsecbugs.txt ;
exit