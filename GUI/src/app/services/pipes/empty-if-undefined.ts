import { Pipe, PipeTransform } from '@angular/core';

@Pipe({
    standalone: true,
    name: "emptyIfUndefined"
})
/**
 * Pipe, die den Wert eines Objekts zurückgibt, wenn dieser nicht undefined ist.
 */
export class EmptyIfUndefinedPipe implements PipeTransform {

    transform(obj: any, path: string): any {
        const value = this.getValue(obj, path);
        return value ?? 'Empty';
    }

    private getValue(obj: any, path: string): any {
        return path.split('.').reduce((o, k) => (o || {})[k], obj);
    }

}
