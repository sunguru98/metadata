use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::*;
use crate::state::*;
use crate::utils::*;

#[derive(Accounts)]
pub struct AppendMetadataItem<'info> {
    #[account()]
    pub update_authority: Signer<'info>,
    #[account(
        mut,
        constraint = metadata.collection.metadata_key_id.eq(&root_collection_metadata_key.id) @ MythicMetadataError::InvalidMetadataKey,
        seeds = [
            PREFIX,
            METADATA,
            root_collection_metadata_key.key().as_ref(),
            metadata.issuing_authority.as_ref(),
            metadata.subject.as_ref()
        ],
        bump
    )]
    pub metadata: Account<'info, Metadata>,
    #[account(
        seeds = [
            PREFIX,
            METADATA_KEY,
            &root_collection_metadata_key.id.to_le_bytes()
        ],
        bump,
    )]
    pub root_collection_metadata_key: Account<'info, MetadataKey>,
    #[account(
        seeds = [
            PREFIX,
            METADATA_KEY,
            &collection_metadata_key.id.to_le_bytes()
        ],
        bump,
    )]
    pub collection_metadata_key: Account<'info, MetadataKey>,
    #[account(
        seeds = [
            PREFIX,
            METADATA_KEY,
            &item_metadata_key.id.to_le_bytes()
        ],
        bump,
    )]
    pub item_metadata_key: Account<'info, MetadataKey>,
}

pub fn handler(ctx: Context<AppendMetadataItem>, args: AppendMetadataItemArgs) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;
    let root_collection_metadata_key = &ctx.accounts.root_collection_metadata_key;
    let collection_metadata_key = &ctx.accounts.collection_metadata_key;
    let item_metadata_key = &ctx.accounts.item_metadata_key;
    let update_authority = &ctx.accounts.update_authority;

    // Check if metadata item is to be appended in root collection
    if check_collection_root_collection_equality(
        root_collection_metadata_key,
        collection_metadata_key,
    ) {
        verify_root_collection_update_authority(&metadata.collection, &update_authority.key())?;

        match metadata
            .collection
            .items
            .binary_search_by_key(&item_metadata_key.id, |item| item.metadata_key_id)
        {
            Ok(_) => return err!(MythicMetadataError::MetadataItemAlreadyExists),
            Err(item_index) => {
                let slot = Clock::get()?.slot;
                metadata.collection.update_slot = slot;
                metadata.collection.items.insert(
                    item_index,
                    MetadataItem {
                        metadata_key_id: item_metadata_key.id,
                        update_slot: slot,
                        value: args.value,
                    },
                );
            }
        };
    } else {
        let (collection_index, mut collection) = verify_collection_update_authority(
            &metadata.collection,
            collection_metadata_key.id,
            &update_authority.key(),
        )?;

        match collection
            .items
            .binary_search_by_key(&item_metadata_key.id, |item| item.metadata_key_id)
        {
            Ok(_) => return err!(MythicMetadataError::MetadataItemAlreadyExists),
            Err(item_index) => {
                let slot = Clock::get()?.slot;
                collection.update_slot = slot;
                collection.items.insert(
                    item_index,
                    MetadataItem {
                        metadata_key_id: item_metadata_key.id,
                        update_slot: slot,
                        value: args.value,
                    },
                );
                metadata.collection.collections.remove(collection_index);
                metadata
                    .collection
                    .collections
                    .insert(collection_index, collection);
            }
        };
    }

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AppendMetadataItemArgs {
    pub value: Vec<u8>,
}
