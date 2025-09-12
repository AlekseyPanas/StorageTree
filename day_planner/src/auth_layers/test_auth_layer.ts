import {LicenseLevel, LicenseToken} from "../auth_server/auth_server_interface";
import {IAuthLayer} from "./auth_layer_interface";

/**
 * For testing the main app, bypasses all auth security by automatically returning true
 */
export class Test_auth_layer implements IAuthLayer {
    is_logged_in(): boolean {
        return true;
    }
    get_session_token(): string | null {
        return "dummy_token";
    }
    get_license(): LicenseToken | null {
        return {
            expiry_unix: 9999999999, level: LicenseLevel.PREMIUM, user_id: "bob@bob.bob"
        }
    }
    submit_id_token(token: any, provider: string): Promise<boolean> {
        return new Promise((resolve, reject) => {
           resolve(true);
        });
    }

}
