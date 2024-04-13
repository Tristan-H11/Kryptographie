/**
 * Interface für Objekte, die mit Standardwerten erstellt werden können.
 */
export interface IDefaultable {
    createDefault(): IDefaultable;
}

/**
 * Interface für Objekte, die mit Standardwerten erstellt werden können und einen Namen besitzen.
 */
export interface IDefaultableWithName extends IDefaultable {
    name: string;
    createDefaultWithName(name: string): IDefaultableWithName;
}
