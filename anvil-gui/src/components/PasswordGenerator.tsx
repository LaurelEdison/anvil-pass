import { useState } from "react";
import { generatePassword } from "../api/password";

interface PasswordGeneratorProps {
  onUse: (password: string) => void;
  onClose: () => void;
}

export function PasswordGenerator({ onUse, onClose }: PasswordGeneratorProps) {
  const [password, setPassword] = useState("");
  const [error, setError] = useState<string | null>(null);

  const [withNumber, setWithNumber] = useState(true);
  const [withUppercase, setWithUppercase] = useState(true);
  const [withLowercase, setWithLowercase] = useState(true);
  const [withSymbols, setWithSymbols] = useState(true);
  const [withExtendedAscii, setWithExtendedAscii] = useState(false);

  async function generate() {
    try {
      const result = await generatePassword({
        withNumber,
        withUppercase,
        withLowercase,
        withSymbols,
        withExtendedAscii,
      });

      setPassword(result.password);
      setError(null);
    } catch (error) {
      setError(String(error));
    }
  }

  return (
    <div className="modal-backdrop">
      <div className="password-generator">
        <h2>Password Generator</h2>

        <input readOnly value={password} placeholder="Generate a password" />

        <div>
          <label>
            <input
              type="checkbox"
              checked={withNumber}
              onChange={(e) => setWithNumber(e.target.checked)}
            />
            Numbers
          </label>

          <label>
            <input
              type="checkbox"
              checked={withUppercase}
              onChange={(e) => setWithUppercase(e.target.checked)}
            />
            Uppercase
          </label>

          <label>
            <input
              type="checkbox"
              checked={withLowercase}
              onChange={(e) => setWithLowercase(e.target.checked)}
            />
            Lowercase
          </label>

          <label>
            <input
              type="checkbox"
              checked={withSymbols}
              onChange={(e) => setWithSymbols(e.target.checked)}
            />
            Symbols
          </label>

          <label>
            <input
              type="checkbox"
              checked={withExtendedAscii}
              onChange={(e) => setWithExtendedAscii(e.target.checked)}
            />
            Extended ASCII
          </label>
        </div>

        {error && <p>{error}</p>}

        <div>
          <button type="button" onClick={generate}>
            Generate
          </button>

          <button
            type="button"
            disabled={!password}
            onClick={() => onUse(password)}
          >
            Use Password
          </button>

          <button type="button" onClick={onClose}>
            Cancel
          </button>
        </div>
      </div>
    </div>
  );
}
