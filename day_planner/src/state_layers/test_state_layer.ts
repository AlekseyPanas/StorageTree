import {AppState, IStateLayerAdapter, Maybe} from "./state_layer_interface";
import {LicenseToken} from "../auth_server/auth_server_interface";
import {IAuthLayer} from "../auth_layers/auth_layer_interface";

export class TestStateLayer implements IStateLayerAdapter {
    private cur_state: AppState;
    private auth_layer: IAuthLayer;
    private cur_on_state_change: (state: AppState) => void;

    constructor(auth_layer: IAuthLayer) {
        this.cur_state = {
            goals: [],
            recurrences: []
        };
        this.auth_layer = auth_layer;
        this.cur_on_state_change = (_) => {};
    }

    /**
        Execute the given operation only if it is permitted, user is logged in, and license is valid.
        Return operation result or appropriate error
     */
    private verify<T>(isPermitted: (license_token: LicenseToken) => boolean, op: () => T): Maybe<T> {
        if (!this.auth_layer.is_logged_in()) { return {
            data: null, license_token_expired_error: false, not_authorized_error: false, not_logged_in_error: true
        } }

        let license = this.auth_layer.get_license();
        if (license === null) { return {
            data: null, license_token_expired_error: true, not_authorized_error: false, not_logged_in_error: false
        } }

        if (!isPermitted(license)) { return {
            data: null, license_token_expired_error: false, not_authorized_error: true, not_logged_in_error: false
        } }

        return {
            data: op(), license_token_expired_error: false, not_authorized_error: false, not_logged_in_error: false
        }
    }

    get_current_state(): Maybe<AppState> {
        return this.verify((_) => true, () => { return this.cur_state; });
    }

    on_state_change(fn: (state: AppState) => void): void {
        this.cur_on_state_change = fn;
    }

}
