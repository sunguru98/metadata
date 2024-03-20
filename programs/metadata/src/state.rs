use anchor_lang::{prelude::*, solana_program::clock::UnixTimestamp};

/// MetadataKey account defines a single metadata item
///
/// PDA seeds: 'metadatakey' + [namespace_authority] + [name]
///
/// Instructions:
///
/// 1) CreateMetadataKey - Creates a new MetadataKey
/// namespace_authority must sign the transaction
///
/// 2) SetCertificationAuthority - Sets the certification authority for the MetadataKey
/// Current certification_authority must sign the transaction
pub struct MetadataKey {
    /// Unique identifier of the MetadataKey assigned by the program
    pub id: u64,

    /// Authority of the MetadataKey namespace
    /// It allows authorities to create unique namespaces for metadata keys
    pub namespace_authority: Pubkey,

    /// Name of the MetadataKey
    /// It must be unique within the namespace authority
    pub name: String,

    /// Description of the MetadataKey
    pub description: String,

    /// The type of the metadata item (e.g. string, number, image, metadata-collection etc.)
    pub content_type: String,

    /// The authority that can certify the metadata item authenticity (optional)
    pub certification_authority: Option<Pubkey>,
}

/// MetadataItem defines a single metadata item identified by its MetadataKey
pub struct MetadataItem {
    /// The id of the key identifying the Metadata item
    pub metadata_key_id: u64,

    /// Serialized metadata item value
    pub value: Vec<u8>,
}

pub struct MetadataCollection {
    /// Unique identifier of the MetadataKey describing the collection
    pub metadata_key_id: u64,

    /// The authority that can update the collection metadata items
    /// If the authority is None then the authority is inherited from parent Metadata account
    pub update_authority: Option<Pubkey>,

    /// Metadata items of the collection
    pub items: Vec<MetadataItem>,

    /// Indicates whether the collection is certified by the certification authority
    /// Any change to the collection metadata invalidates its certification status
    pub certified: bool,

    /// The time when the certification expires (optional)
    /// Or None if the collection is certified indefinitely
    pub certification_expiry: Option<UnixTimestamp>,

    /// The slot when the collection was last updated
    pub update_slot: u64,
}

/// Metadata account defines a set of metadata items
///
// PDA seeds: 'metadata' + [metadata_key_id] + [subject] + [create_authority]
pub struct Metadata {
    /// Unique identifier of the MetadataKey describing the collection
    pub metadata_key_id: u64,

    /// The subject described by the metadata (e.g. a DAO, NFT, a program etc.)
    pub subject: Pubkey,

    /// The default update authority for all the collections
    /// Note: The authority can be overridden at the collection level
    pub update_authority: Option<Pubkey>,

    /// A set of metadata collections  
    pub collections: Vec<MetadataCollection>,
}