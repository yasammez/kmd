//! Kernel Objects.

use ::basedef::*;
use ::device_object::PDEVICE_OBJECT;
use ::irp::IRP;
use ::status::NTSTATUS;
use ::string::UNICODE_STRING;

extern "fastcall"
{
	pub fn KeWaitForSingleObject(Object: PVOID, WaitReason: u32, WaitMode: KPROCESSOR_MODE, Alertable: bool, Timeout: Option<&i64>) -> NTSTATUS;
    pub fn ObfDereferenceObject(Object: *const km_void);
    pub fn ObReferenceObjectByName(ObjectPath: &UNICODE_STRING, Attributes: u32, PassedAccessState: *const ACCESS_STATE, DesiredAccess: u32, ObjectType: *const OBJECT_TYPE, AccessMode: KPROCESSOR_MODE, ParseContext: *const km_void, ObjectPtr: *mut km_void) -> NTSTATUS;
}

#[repr(C)]
pub struct ACCESS_STATE {
    OperationID: u64, // LUID?
    /* ... */
}

#[repr(C)]
pub struct OBJECT_TYPE {
    dummy: u8
}

#[repr(C)]
pub struct WAIT_CONTEXT_BLOCK
{
	WaitQueueEntry: *mut KDEVICE_QUEUE_ENTRY,
	DeviceRoutine: extern "system" fn (_obj: PDEVICE_OBJECT, _irp: *mut IRP, *mut u8, *mut u8) -> IO_ALLOCATION_ACTION,
	DeviceContext: *mut u8,
	NumberOfMapRegisters: u32,
	DeviceObject: *mut u8,
	CurrentIrp: *mut u8,
	BufferChainingDpc: * mut u8,
}

#[repr(C)]
pub enum IO_ALLOCATION_ACTION
{
	KeepObject = 0x01,
	DeallocateObject = 0x02,
	DeallocateObjectKeepRegisters = 0x03,
}

#[repr(C)]
pub struct KDEVICE_QUEUE_ENTRY
{
	DeviceListEntry: LIST_ENTRY,
	SortKey: u32,
	Inserted: bool,
}

#[repr(C)]
pub struct KDEVICE_QUEUE
{
	Type: u16,
	Size: u16,
	DeviceListHead: LIST_ENTRY,
	Lock: KSPIN_LOCK,
	Busy: bool,
}


/// Object type pointers
#[link(name = "ntoskrnl")]
extern {
    pub static IoDriverObjectType: *const OBJECT_TYPE;
    pub static CmKeyObjectType: *const OBJECT_TYPE;
    pub static IoFileObjectType: *const OBJECT_TYPE;
    pub static ExEventObjectType: *const OBJECT_TYPE;
    pub static ExSemaphoreObjectType: *const OBJECT_TYPE;
    pub static TmTransactionManagerObjectType: *const OBJECT_TYPE;
    pub static TmResourceManagerObjectType: *const OBJECT_TYPE;
    pub static TmEnlistmentObjectType: *const OBJECT_TYPE;
    pub static TmTransactionObjectType: *const OBJECT_TYPE;
    pub static PsProcessType: *const OBJECT_TYPE;
    pub static PsThreadType: *const OBJECT_TYPE;
    pub static PsJobType: *const OBJECT_TYPE;
    pub static SeTokenObjectType: *const OBJECT_TYPE;
}


pub const OBJ_INHERIT: u32 =   			            0x00000002;
pub const OBJ_PERMANENT: u32 =          			0x00000010;
pub const OBJ_EXCLUSIVE: u32 =          			0x00000020;
pub const OBJ_CASE_INSENSITIVE: u32 =   			0x00000040;
pub const OBJ_OPENIF: u32 =             			0x00000080;
pub const OBJ_OPENLINK: u32 =           			0x00000100;
pub const OBJ_KERNEL_HANDLE: u32 =      			0x00000200;
pub const OBJ_FORCE_ACCESS_CHECK: u32 = 			0x00000400;
pub const OBJ_IGNORE_IMPERSONATED_DEVICEMAP: u32 =	0x00000800;
pub const OBJ_VALID_ATTRIBUTES: u32 =   			0x00000FF2;
