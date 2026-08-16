export interface EntryDto {
	id: string;
	parent: string;
	title: string;
	username: string;
	password: string;
	url: string;
	notes: string;
	totp: string;
}

export interface GroupDto {
	id: string;
	parent: string | null;
	name: string;
	tags: string[];
	notes: string;
}
