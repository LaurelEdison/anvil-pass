import { useState } from "react";
import { EntryDto } from "../types/types";
import { updateEntry } from "../api/entry";
import { PasswordGenerator } from "../components/PasswordGenerator";

interface EditEntryPageProps {
	entry: EntryDto;
	onBack: () => void;
}

export function EditEntryPage({
	entry,
	onBack,
}: EditEntryPageProps) {
	const [title, setTitle] = useState(entry.title);
	const [username, setUsername] = useState(entry.username);
	const [password, setPassword] = useState(entry.password);
	const [url, setUrl] = useState(entry.url);
	const [notes, setNotes] = useState(entry.notes);
	const [totp, setTotp] = useState(entry.totp);
	const [showPassword, setShowPassword] = useState(false);

	const [showPasswordGenerator, setShowPasswordGenerator] =
		useState(false);

	const [saving, setSaving] = useState(false);
	const [error, setError] = useState<string | null>(null);

	const hasChanges =
		title !== entry.title ||
		username !== entry.username ||
		password !== entry.password ||
		url !== entry.url ||
		notes !== entry.notes ||
		totp !== entry.totp;

	async function handleSave() {
		if (!hasChanges || saving) {
			return;
		}

		setSaving(true);
		setError(null);

		try {
			await updateEntry({
				id: entry.id,
				title,
				username,
				password,
				url,
				notes,
				totp,
			});

			onBack();
		} catch (error) {
			setError(String(error));
		} finally {
			setSaving(false);
		}
	}

	return (
		<main>
			<header>
				<button type="button" onClick={onBack}>
					Back
				</button>

				<h1>Edit Entry</h1>
			</header>

			<form
				onSubmit={(e) => {
					e.preventDefault();
					handleSave();
				}}
			>
				<div>
					<label htmlFor="entry-title">
						Title
					</label>

					<input
						id="entry-title"
						value={title}
						onChange={(e) =>
							setTitle(e.target.value)
						}
					/>
				</div>

				<div>
					<label htmlFor="entry-username">
						Username
					</label>

					<input
						id="entry-username"
						value={username}
						onChange={(e) =>
							setUsername(e.target.value)
						}
					/>
				</div>

				<div>
					<label htmlFor="entry-password">
						Password
					</label>

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
								onClick={() => setShowPassword((value) => !value)}
							>
								{showPassword ? "Hide" : "Show"}
							</button>

							<button
								type="button"
								onClick={() => setShowPasswordGenerator(true)}
							>
								Generate Password
							</button>
						</div>
					</div>
				</div>

				<div>
					<label htmlFor="entry-url">
						URL
					</label>

					<input
						id="entry-url"
						value={url}
						onChange={(e) =>
							setUrl(e.target.value)
						}
					/>
				</div>

				<div>
					<label htmlFor="entry-totp">
						TOTP
					</label>

					<input
						id="entry-totp"
						value={totp}
						onChange={(e) =>
							setTotp(e.target.value)
						}
					/>
				</div>

				<div>
					<label htmlFor="entry-notes">
						Notes
					</label>

					<textarea
						id="entry-notes"
						value={notes}
						onChange={(e) =>
							setNotes(e.target.value)
						}
					/>
				</div>

				{error && <p>{error}</p>}

				<button
					type="submit"
					disabled={!hasChanges || saving}
				>
					{saving ? "Saving..." : "Save"}
				</button>
			</form>

			{showPasswordGenerator && (
				<div>
					<div>
						<PasswordGenerator
							onUse={(generatedPassword) => {
								setPassword(generatedPassword);
								setShowPasswordGenerator(false);
							}}
							onClose={() => setShowPasswordGenerator(false)}
						/>
					</div>
				</div>
			)}
		</main>
	);
}
