#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `OPERATION_DONE` reader - Operation done"]
pub type OperationDoneR = crate::BitReader;
#[doc = "Field `OPERATION_DONE` writer - Operation done"]
pub type OperationDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `READ_NUM_DONE` reader - Read num done"]
pub type ReadNumDoneR = crate::BitReader;
#[doc = "Field `READ_NUM_DONE` writer - Read num done"]
pub type ReadNumDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROG_DATA_WAIT` reader - Prog data wait"]
pub type ProgDataWaitR = crate::BitReader;
#[doc = "Field `PROG_DATA_WAIT` writer - Prog data wait"]
pub type ProgDataWaitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHBUSY_ERR` reader - Flash busy error"]
pub type FlashbusyErrR = crate::BitReader;
#[doc = "Field `FLASHBUSY_ERR` writer - Flash busy error"]
pub type FlashbusyErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTION_WR_ERR` reader - Option write error"]
pub type OptionWrErrR = crate::BitReader;
#[doc = "Field `OPTION_WR_ERR` writer - Option write error"]
pub type OptionWrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAGE_ERASE_ERR` reader - Page erase error"]
pub type PageEraseErrR = crate::BitReader;
#[doc = "Field `PAGE_ERASE_ERR` writer - Page erase error"]
pub type PageEraseErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROG_ERR` reader - Prog error"]
pub type ProgErrR = crate::BitReader;
#[doc = "Field `PROG_ERR` writer - Prog error"]
pub type ProgErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONE_BIT_CORRECT` reader - One bit correct"]
pub type OneBitCorrectR = crate::BitReader;
#[doc = "Field `ONE_BIT_CORRECT` writer - One bit correct"]
pub type OneBitCorrectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWO_BIT_ERROR` reader - Two bit error"]
pub type TwoBitErrorR = crate::BitReader;
#[doc = "Field `TWO_BIT_ERROR` writer - Two bit error"]
pub type TwoBitErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Operation done"]
    #[inline(always)]
    pub fn operation_done(&self) -> OperationDoneR {
        OperationDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read num done"]
    #[inline(always)]
    pub fn read_num_done(&self) -> ReadNumDoneR {
        ReadNumDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Prog data wait"]
    #[inline(always)]
    pub fn prog_data_wait(&self) -> ProgDataWaitR {
        ProgDataWaitR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Flash busy error"]
    #[inline(always)]
    pub fn flashbusy_err(&self) -> FlashbusyErrR {
        FlashbusyErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Option write error"]
    #[inline(always)]
    pub fn option_wr_err(&self) -> OptionWrErrR {
        OptionWrErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Page erase error"]
    #[inline(always)]
    pub fn page_erase_err(&self) -> PageEraseErrR {
        PageEraseErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Prog error"]
    #[inline(always)]
    pub fn prog_err(&self) -> ProgErrR {
        ProgErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - One bit correct"]
    #[inline(always)]
    pub fn one_bit_correct(&self) -> OneBitCorrectR {
        OneBitCorrectR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Two bit error"]
    #[inline(always)]
    pub fn two_bit_error(&self) -> TwoBitErrorR {
        TwoBitErrorR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operation done"]
    #[inline(always)]
    pub fn operation_done(&mut self) -> OperationDoneW<'_, SrSpec> {
        OperationDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Read num done"]
    #[inline(always)]
    pub fn read_num_done(&mut self) -> ReadNumDoneW<'_, SrSpec> {
        ReadNumDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - Prog data wait"]
    #[inline(always)]
    pub fn prog_data_wait(&mut self) -> ProgDataWaitW<'_, SrSpec> {
        ProgDataWaitW::new(self, 2)
    }
    #[doc = "Bit 3 - Flash busy error"]
    #[inline(always)]
    pub fn flashbusy_err(&mut self) -> FlashbusyErrW<'_, SrSpec> {
        FlashbusyErrW::new(self, 3)
    }
    #[doc = "Bit 4 - Option write error"]
    #[inline(always)]
    pub fn option_wr_err(&mut self) -> OptionWrErrW<'_, SrSpec> {
        OptionWrErrW::new(self, 4)
    }
    #[doc = "Bit 5 - Page erase error"]
    #[inline(always)]
    pub fn page_erase_err(&mut self) -> PageEraseErrW<'_, SrSpec> {
        PageEraseErrW::new(self, 5)
    }
    #[doc = "Bit 6 - Prog error"]
    #[inline(always)]
    pub fn prog_err(&mut self) -> ProgErrW<'_, SrSpec> {
        ProgErrW::new(self, 6)
    }
    #[doc = "Bit 7 - One bit correct"]
    #[inline(always)]
    pub fn one_bit_correct(&mut self) -> OneBitCorrectW<'_, SrSpec> {
        OneBitCorrectW::new(self, 7)
    }
    #[doc = "Bit 8 - Two bit error"]
    #[inline(always)]
    pub fn two_bit_error(&mut self) -> TwoBitErrorW<'_, SrSpec> {
        TwoBitErrorW::new(self, 8)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
