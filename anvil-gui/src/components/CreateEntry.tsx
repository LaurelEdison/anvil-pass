import { useState } from "react";
import { createEntry } from "../api/entry";
import { PasswordGenerator } from "./PasswordGenerator";

interface CreateEntryProps {
	parent: string | null;
	onCreated: () => void;
	onCancel: () => void;
	onDirty: () => void;
}

export function CreateEntry({
	parent,
	onCreated,
	onCancel,
	onDirty,
}: CreateEntryProps) {
	const [title, setTitle] = useState("");
	const [username, setUsername] = useState("");
	const [password, setPassword] = useState("");
	const [url, setUrl] = useState("");
	const [notes, setNotes] = useState("");
	const [totp, setTotp] = useState("");

	const [showPassword, setShowPassword] = useState(false);
	const [showGenerator, setShowGenerator] = useState(false);

	const [saving, setSaving] = useState(false);
	const [error, setError] = useState<string | null>(null);

	async function handleCreate() {
		if (!password || saving) {
			return;
		}

		setSaving(true);
		setError(null);

		try {
			await createEntry({
				parent,
				title: title || null,
				username: username || null,
				password,
				url: url || null,
				notes: notes || null,
				totp: totp || null,
			});

			onDirty();
			onCreated();
		} catch (error) {
			setError(String(error));
		} finally {
			setSaving(false);
		}
	}

	return (
		<section className="entry-create">
			<h2>Create Entry</h2>

			<div>
				<label htmlFor="entry-title">
					Title
				</label>

				<input
					id="entry-title"
					value={title}
					onChange={(e) => setTitle(e.target.value)}
				/>
			</div>

			<div>
				<label htmlFor="entry-username">
					Username
				</label>

				<input
					id="entry-username"
					value={username}
					onChange={(e) => setUsername(e.target.value)}
				/>
			</div>

			<div>
				<label htmlFor="entry-password">
					Password
				</label>

				<div>
					<input
						id="entry-password"
						type={showPassword ? "text" : "password"}
						value={password}
						onChange={(e) => setPassword(e.target.value)}
					/>

					<button
						type="button"
						onClick={() =>
							setShowPassword((value) => !value)
						}
					>
						{showPassword ? "Hide" : "Show"}
					</button>

					<button
						type="button"
						onClick={() => setShowGenerator(true)}
					>
						Generate Password
					</button>
				</div>
			</div>

			<div>
				<label htmlFor="entry-url">
					URL
				</label>

				<input
					id="entry-url"
					value={url}
					onChange={(e) => setUrl(e.target.value)}
				/>
			</div>

			<div>
				<label htmlFor="entry-totp">
					TOTP
				</label>

				<input
					id="entry-totp"
					value={totp}
					onChange={(e) => setTotp(e.target.value)}
				/>
			</div>

			<div>
				<label htmlFor="entry-notes">
					Notes
				</label>

				<textarea
					id="entry-notes"
					value={notes}
					onChange={(e) => setNotes(e.target.value)}
				/>
			</div>

			{error && <p>{error}</p>}

			<div>
				<button type="button" onClick={onCancel}>
					Cancel
				</button>

				<button
					type="button"
					disabled={!password || saving}
					onClick={handleCreate}
				>
					{saving ? "Creating..." : "Create Entry"}
				</button>
			</div>

			{showGenerator && (
				<PasswordGenerator
					onUse={(generatedPassword) => {
						setPassword(generatedPassword);
						setShowGenerator(false);
					}}
					onClose={() => setShowGenerator(false)}
				/>
			)}
		</section>
	);
}


