/**
 * Schnittstelle für Nachrichten besteht nur aus einem String.
 */
export class SingleMessageModel {
	message: string;

	constructor(message: string) {
		this.message = message;
	}
}
