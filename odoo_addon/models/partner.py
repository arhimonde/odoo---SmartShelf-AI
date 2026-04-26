from odoo import models, fields

class ResPartner(models.Model):
    _inherit = 'res.partner'

    jetson_note = fields.Char(string='Nota Jetson')
