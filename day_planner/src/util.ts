/**
 * Return if the two provided intervals intersect each other on the number line
 */
export function do_bounds_intersect(start1: number, end1: number, start2: number, end2: number): boolean {
    if (end1 <= start1 || end2 <= start2) { return false; }
    return (start2 <= start1 && start1 <= end2) || (start2 <= end1 && end1 <= end2) ||
        ((start1 <= start2 && start2 <= end1) && (start1 <= end2 && end2 <= end1));
}
