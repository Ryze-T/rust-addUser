use windows::{
    Win32::NetworkManagement::NetManagement::USER_INFO_1,
    Win32::NetworkManagement::NetManagement::NetUserAdd,
    Win32::NetworkManagement::NetManagement::UF_SCRIPT,
    Win32::NetworkManagement::NetManagement::USER_PRIV_USER,
    Win32::NetworkManagement::NetManagement::NetLocalGroupAddMembers,
    core::PWSTR,
    core::PCWSTR
};
use windows::Win32::NetworkManagement::NetManagement::LOCALGROUP_MEMBERS_INFO_3;

fn main()  {
    let servename = PCWSTR::default();
    let level:u32 = 1;
    let parm_error= &mut 0;

    let mut username: Vec<u16> = "test".encode_utf16().collect();
    username.push(0);
    let p_username = username.as_ptr() as *mut u16;
    let mut password: Vec<u16> = "1q@W3e$r".encode_utf16().collect();
    password.push(0);
    let p_password = password.as_ptr() as *mut u16;

    let mut groupname: Vec<u16> = "administrators".encode_utf16().collect();
    groupname.push(0);
    let p_groupname = groupname.as_ptr() as *mut u16;

    let ui1 = &mut USER_INFO_1{
        usri1_name:PWSTR(p_username),
        usri1_password:PWSTR(p_password),
        usri1_password_age: 0,
        usri1_priv: USER_PRIV_USER,
        usri1_home_dir: PWSTR(std::ptr::null_mut()),
        usri1_comment: PWSTR(std::ptr::null_mut()),
        usri1_flags: UF_SCRIPT,
        usri1_script_path: PWSTR(std::ptr::null_mut()),
    };

    let lmi3  = &mut LOCALGROUP_MEMBERS_INFO_3{
        lgrmi3_domainandname: PWSTR(p_username),
    };

    unsafe{
        let result = NetUserAdd(servename,level,ui1 as *const _ as _,parm_error );
        if result == 0 {
            println!("[+] User added successfully");
        }
        else {
            println!("[-] Failed to add user:{}", result);
        }
        let result2 = NetLocalGroupAddMembers(servename,PCWSTR(p_groupname),3,lmi3 as *const _ as _,1);
        if result2 == 0 {
            println!("[+] Administrators group added successfully");
        }
        else{
            println!("[-] Failed to add Administrators group:{}",result2);
        }
    }

}